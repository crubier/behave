//! Parallel composite -- ticks ALL children each cycle.
//! Succeeds when all succeed, fails on first failure.

use log::info;

use super::{ActionIO, ActionRun, ActionResult, TickResult, action_result};

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.parallel.rs"));
}

pub fn tick(run: &mut ActionRun, io: &ActionIO) -> TickResult {
    let id = run.run_id;
    let child_count = run.children.len();
    let mut succeeded = 0;
    let mut failed = 0;

    for child in &mut run.children {
        match super::tick(child, io) {
            TickResult::Running => {}
            TickResult::Success => succeeded += 1,
            TickResult::Failure => failed += 1,
        }
    }

    if failed > 0 {
        info!("[#{id}] PARALLEL failed ({failed} failed, {succeeded} succeeded)");
        run.result = Some(ActionResult { result: Some(action_result::Result::Parallel(
            proto::ParallelResult { success: false, succeeded_count: succeeded, failed_count: failed, child_count: child_count as u32 },
        ))});
        TickResult::Failure
    } else if succeeded as usize == child_count {
        info!("[#{id}] PARALLEL all {child_count} children succeeded");
        run.result = Some(ActionResult { result: Some(action_result::Result::Parallel(
            proto::ParallelResult { success: true, succeeded_count: succeeded, failed_count: 0, child_count: child_count as u32 },
        ))});
        TickResult::Success
    } else {
        TickResult::Running
    }
}
