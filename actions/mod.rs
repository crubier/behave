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

pub use io::ActionIO;

pub mod action_proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.rs"));
}

pub use action_proto::{
    action_args, action_result, ActionArgs, ActionResult, ActionRun, ActionState, RunStatus,
};

// ── Tick result (control flow only -- data goes into ActionRun) ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickResult {
    Running,
    Success,
    Failure,
}

// ── Time ────────────────────────────────────────────────────────

pub fn now_utime() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u64
}

// ── Create a run tree from args ─────────────────────────────────

static RUN_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

fn next_run_id() -> u64 {
    RUN_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// Create an ActionRun tree from an ActionArgs tree.
/// Each run gets a unique run_id and starts in PENDING status.
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
        children: args.children.iter().map(|c| init_run(c)).collect(),
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

// ── Tick dispatch ───────────────────────────────────────────────

/// Tick an ActionRun. Returns Running/Success/Failure for control flow.
/// All data (result, state, status, timing) is written into the run.
pub fn tick(run: &mut ActionRun, io: &ActionIO) -> TickResult {
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

    let result = match action {
        Some(action_args::Action::Sequence(_)) => sequence::tick(run, io),
        Some(action_args::Action::Fallback(_)) => fallback::tick(run, io),
        Some(action_args::Action::Parallel(_)) => parallel::tick(run, io),
        Some(action_args::Action::Concurrent(ref a)) => concurrent::tick(run, a, io),
        Some(action_args::Action::Loop(ref a)) => loop_action::tick(run, a, io),
        Some(action_args::Action::Takeoff(ref a)) => takeoff::tick(run, a, io),
        Some(action_args::Action::Land(ref a)) => land::tick(run, a, io),
        Some(action_args::Action::GotoWaypoint(ref a)) => goto_waypoint::tick(run, a, io),
        Some(action_args::Action::ReturnHome(ref a)) => return_home::tick(run, a, io),
        Some(action_args::Action::TakePhoto(_)) => take_photo::tick(run, io),
        None => TickResult::Failure,
    };

    // Mark completion
    match result {
        TickResult::Success => {
            run.status = RunStatus::Succeeded.into();
            run.ended_at = now_utime();
        }
        TickResult::Failure => {
            run.status = RunStatus::Failed.into();
            run.ended_at = now_utime();
        }
        TickResult::Running => {}
    }

    result
}
