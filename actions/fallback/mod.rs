//! Fallback composite -- ticks children left-to-right, succeeds if ANY succeeds.

use log::info;

use super::{ActionIO, ActionRun, ActionResult, TickResult, action_result};

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.fallback.rs"));
}

pub fn tick(run: &mut ActionRun, io: &ActionIO) -> TickResult {
    let id = run.run_id;
    let child_count = run.children.len();

    // Find current child (first non-failed)
    let idx = run.children.iter()
        .position(|c| c.status != super::RunStatus::Failed as i32)
        .unwrap_or(child_count);

    if idx >= child_count {
        info!("[#{id}] FALLBACK all {child_count} children failed");
        run.result = Some(ActionResult { result: Some(action_result::Result::Fallback(
            proto::FallbackResult { success: false, succeeded_at_index: -1, children_attempted: child_count as u32 },
        ))});
        return TickResult::Failure;
    }

    let child_result = super::tick(&mut run.children[idx], io);

    match child_result {
        TickResult::Running => TickResult::Running,
        TickResult::Success => {
            info!("[#{id}] FALLBACK child {idx} succeeded -- done");
            run.result = Some(ActionResult { result: Some(action_result::Result::Fallback(
                proto::FallbackResult { success: true, succeeded_at_index: idx as i32, children_attempted: (idx + 1) as u32 },
            ))});
            TickResult::Success
        }
        TickResult::Failure => {
            info!("[#{id}] FALLBACK child {idx} failed, trying next");
            if idx + 1 >= child_count {
                info!("[#{id}] FALLBACK all children failed");
                run.result = Some(ActionResult { result: Some(action_result::Result::Fallback(
                    proto::FallbackResult { success: false, succeeded_at_index: -1, children_attempted: child_count as u32 },
                ))});
                TickResult::Failure
            } else {
                TickResult::Running
            }
        }
    }
}
