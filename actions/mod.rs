//! Action tree runtime.
//!
//! Uses protobuf types directly:
//! - `ActionArgs` is the mission definition (tree of actions)
//! - `ActionRun` is the execution state (mutated during ticks)
//!
//! No hand-written wrapper enums. The proto oneof IS the dispatch.

pub mod io;
pub mod sequence;
pub mod fallback;
pub mod parallel;
pub mod concurrent;
pub mod takeoff;
pub mod goto_waypoint;
pub mod return_home;
pub mod land;
pub mod take_photo;

#[path = "loop/mod.rs"]
pub mod loop_action;

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use log::info;
use prost::Message;

pub use io::ActionAPI;

pub mod action_proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.rs"));
}

pub use action_proto::{
    action_args, action_input, action_input_envelope, action_output, action_result, action_state,
    ActionArgs, ActionInput, ActionInputEnvelope, ActionOutput, ActionOutputEnvelope,
    ActionResult, ActionResultEnvelope, ActionRun, ActionState, RunStatus,
};

// ── Tick result (control flow only -- data goes into ActionRun) ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickResult {
    Running,
    Success,
    Failure,
}

// ── ActionRun accessor macros ────────────────────────────────────
//
// These extract/set the action-specific typed data from/into the
// oneof wrappers in ActionRun, removing boilerplate from actions.

/// Extract action-specific args from an ActionRun.
/// Returns the args reference, or returns `TickResult::Failure` from
/// the calling function if the variant doesn't match.
macro_rules! get_args {
    ($run:expr, $variant:ident) => {
        match $run.args.as_ref().and_then(|a| a.action.as_ref()) {
            Some($crate::actions::action_args::Action::$variant(ref a)) => a,
            _ => return $crate::actions::TickResult::Failure,
        }
    };
}
pub(crate) use get_args;

/// Set the action-specific result on an ActionRun and notify via ActionAPI.
macro_rules! set_result {
    ($api:expr, $run:expr, $variant:ident, $value:expr) => {{
        let result = $crate::actions::ActionResult {
            result: Some($crate::actions::action_result::Result::$variant($value)),
        };
        $run.result = Some(result.clone());
        let _ = $api.result_tx.send($crate::actions::ActionResultEnvelope {
            action_args_id: $crate::actions::action_id($run),
            action_run_id: $run.run_id,
            status: $run.status,
            result: Some(result),
        });
    }};
}
pub(crate) use set_result;

/// Set the action-specific state on an ActionRun.
macro_rules! set_state {
    ($run:expr, $variant:ident, $value:expr) => {
        $run.state = Some($crate::actions::ActionState {
            state: Some($crate::actions::action_state::State::$variant($value)),
        });
    };
}
pub(crate) use set_state;

/// Get the action-specific state from an ActionRun. Returns `Option<&T>`.
macro_rules! get_state {
    ($run:expr, $variant:ident) => {
        match $run.state.as_ref().and_then(|s| s.state.as_ref()) {
            Some($crate::actions::action_state::State::$variant(ref s)) => Some(s),
            _ => None,
        }
    };
}
pub(crate) use get_state;

/// Push an action-specific output onto the ActionRun outputs list and notify via ActionAPI.
macro_rules! push_output {
    ($api:expr, $run:expr, $variant:ident, $value:expr) => {{
        let output = $crate::actions::ActionOutput {
            output: Some($crate::actions::action_output::Output::$variant($value)),
        };
        $run.outputs.push(output.clone());
        let _ = $api.output_tx.send($crate::actions::ActionOutputEnvelope {
            action_args_id: $crate::actions::action_id($run),
            action_run_id: $run.run_id,
            output: Some(output),
        });
    }};
}
pub(crate) use push_output;

/// Get the latest action-specific input from an ActionRun. Returns `Option<&T>`.
macro_rules! pull_input {
    ($run:expr, $variant:ident) => {
        $run.inputs.last().and_then(|i| match i.input.as_ref() {
            Some($crate::actions::action_input::Input::$variant(ref v)) => Some(v),
            _ => None,
        })
    };
}
pub(crate) use pull_input;

// ── Time ────────────────────────────────────────────────────────

pub fn now_utime() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u64
}

// ── Create a run tree from args ─────────────────────────────────

/// Generate a unique run ID. Uses timestamp (microseconds) in the
/// upper 44 bits and a counter in the lower 20 bits, giving ~1M
/// unique IDs per microsecond across restarts without collisions.
fn next_run_id() -> u64 {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let seq = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed) & 0xF_FFFF; // 20 bits
    let ts = now_utime() & 0xFFF_FFFF_FFFF; // 44 bits (~557 years)
    (ts << 20) | seq
}

/// Create an ActionRun tree from an ActionArgs tree.
/// Each run gets a unique run_id and starts in PENDING status.
/// Create a single ActionRun for an ActionArgs. No children --
/// each composite creates its own child runs when it starts.
pub fn init_run(args: &ActionArgs) -> ActionRun {
    ActionRun {
        run_id: next_run_id(),
        args: Some(args.clone()),
        started_at: 0,
        ended_at: 0,
        status: RunStatus::Pending.into(),
        result: None,
        state: None,
        outputs: vec![],
        inputs: vec![],
        children: vec![],
        action_type_id: 0,
    }
}

/// Decode ActionArgs from bytes and create an ActionRun.
pub fn from_bytes(bytes: &[u8]) -> Result<ActionRun> {
    let args = ActionArgs::decode(bytes)?;
    let args_id = args.id;
    let run = init_run(&args);
    info!("built run #{} for action #{args_id} ({} children)", run.run_id, run.children.len());
    Ok(run)
}

/// Encode an ActionRun tree to protobuf bytes.
pub fn to_bytes(run: &ActionRun) -> Vec<u8> {
    run.encode_to_vec()
}

// ── Action type ID ──────────────────────────────────────────────
//
// Each action type has a fixed numeric ID matching its protobuf field
// number across all five oneofs (Args=2..11, Result, State, Input, Output).
// The ID is stored on ActionRun.action_type_id and set once by tick().

/// Get the action type ID from an ActionRun.
/// Returns the shared protobuf field number (2-11), or 0 if not yet ticked.
pub fn action_type_id(run: &ActionRun) -> u32 {
    run.action_type_id
}

/// Get the ActionArgs.id from an ActionRun.
pub fn action_id(run: &ActionRun) -> u64 {
    run.args.as_ref().map(|a| a.id).unwrap_or(0)
}

// ── Input routing ───────────────────────────────────────────────

/// Route an ActionInputEnvelope to matching ActionRun(s) in the tree.
/// Pushes the input into each matching run's `inputs` vector.
pub fn route_input(run: &mut ActionRun, envelope: &ActionInputEnvelope) {
    let matches = match &envelope.target {
        Some(action_input_envelope::Target::ActionRunId(id)) => run.run_id == *id,
        Some(action_input_envelope::Target::ActionArgsId(id)) => action_id(run) == *id,
        Some(action_input_envelope::Target::ActionTypeId(id)) => action_type_id(run) == *id,
        None => false,
    };

    if matches {
        if let Some(input) = &envelope.input {
            run.inputs.push(input.clone());
        }
    }

    for child in &mut run.children {
        route_input(child, envelope);
    }
}

// ── Tick dispatch ───────────────────────────────────────────────

/// Tick an ActionRun. Returns Running/Success/Failure for control flow.
/// All data (result, state, status, timing) is written into the run.
pub fn tick(api: &ActionAPI, run: &mut ActionRun) -> TickResult {
    // Already completed? Return cached result.
    match RunStatus::try_from(run.status) {
        Ok(RunStatus::Succeeded) => return TickResult::Success,
        Ok(RunStatus::Failed) => return TickResult::Failure,
        _ => {}
    }

    // First tick? Mark as running.
    if run.status == RunStatus::Pending as i32 {
        run.status = RunStatus::Running.into();
        run.started_at = now_utime();
    }

    let action = run.args.as_ref().and_then(|a| a.action.as_ref()).cloned();

    let (type_id, result) = match action {
        Some(action_args::Action::Sequence(_))     => ( 2, sequence::tick(api, run)),
        Some(action_args::Action::Fallback(_))     => ( 3, fallback::tick(api, run)),
        Some(action_args::Action::Takeoff(_))      => ( 4, takeoff::tick(api, run)),
        Some(action_args::Action::Land(_))         => ( 5, land::tick(api, run)),
        Some(action_args::Action::GotoWaypoint(_)) => ( 6, goto_waypoint::tick(api, run)),
        Some(action_args::Action::ReturnHome(_))   => ( 7, return_home::tick(api, run)),
        Some(action_args::Action::TakePhoto(_))    => ( 8, take_photo::tick(api, run)),
        Some(action_args::Action::Parallel(_))     => ( 9, parallel::tick(api, run)),
        Some(action_args::Action::Loop(_))         => (10, loop_action::tick(api, run)),
        Some(action_args::Action::Concurrent(_))   => (11, concurrent::tick(api, run)),
        None => (0, TickResult::Failure),
    };
    run.action_type_id = type_id;

    // Mark completion and notify
    match result {
        TickResult::Success => {
            run.status = RunStatus::Succeeded.into();
            run.ended_at = now_utime();
            if let Some(r) = &run.result {
                let _ = api.result_tx.send(ActionResultEnvelope {
                    action_args_id: action_id(run),
                    action_run_id: run.run_id,
                    status: run.status,
                    result: Some(r.clone()),
                });
            }
        }
        TickResult::Failure => {
            run.status = RunStatus::Failed.into();
            run.ended_at = now_utime();
            if let Some(r) = &run.result {
                let _ = api.result_tx.send(ActionResultEnvelope {
                    action_args_id: action_id(run),
                    action_run_id: run.run_id,
                    status: run.status,
                    result: Some(r.clone()),
                });
            }
        }
        TickResult::Running => {}
    }

    result
}
