//! Sequence composite -- ticks children left-to-right, succeeds if ALL succeed.

use log::info;

use super::{ActionIO, ActionRun, ActionResult, TickResult, action_result};

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.sequence.rs"));
}

pub fn tick(run: &mut ActionRun, io: &ActionIO) -> TickResult {
    let id = run.run_id;

    // Create child runs on first tick
    if run.children.is_empty() {
        if let Some(args) = &run.args {
            run.children = args.children.iter().map(|c| super::init_run(c)).collect();
        }
        let child_count = run.children.len();
        info!("[#{id}] SEQUENCE start ({child_count} children)");
    }

    let child_count = run.children.len();

    // Find current child (first non-succeeded)
    let idx = run.children.iter()
        .position(|c| c.status != super::RunStatus::Succeeded as i32)
        .unwrap_or(child_count);

    if idx >= child_count {
        run.result = Some(ActionResult { result: Some(action_result::Result::Sequence(
            proto::SequenceResult { success: true, children_completed: child_count as u32, failed_at_index: -1 },
        ))});
        return TickResult::Success;
    }

    let child_result = super::tick(&mut run.children[idx], io);

    match child_result {
        TickResult::Running => {
            info!("[#{id}] SEQUENCE running child {idx}/{child_count}");
            TickResult::Running
        }
        TickResult::Success => {
            info!("[#{id}] SEQUENCE child {idx} succeeded ({}/{})", idx + 1, child_count);
            if idx + 1 >= child_count {
                info!("[#{id}] SEQUENCE all children done");
                run.result = Some(ActionResult { result: Some(action_result::Result::Sequence(
                    proto::SequenceResult { success: true, children_completed: child_count as u32, failed_at_index: -1 },
                ))});
                TickResult::Success
            } else {
                TickResult::Running
            }
        }
        TickResult::Failure => {
            info!("[#{id}] SEQUENCE child {idx} FAILED -- aborting");
            run.result = Some(ActionResult { result: Some(action_result::Result::Sequence(
                proto::SequenceResult { success: false, children_completed: idx as u32, failed_at_index: idx as i32 },
            ))});
            TickResult::Failure
        }
    }
}
