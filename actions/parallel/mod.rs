//! Parallel composite -- ticks ALL children each cycle.
//! Succeeds when all succeed, fails on first failure.

use log::info;

use super::{ActionAPI, ActionRun, ActionResult, TickResult, action_result};

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.parallel.rs"));
}

pub fn tick(api: &ActionAPI, run: &mut ActionRun) -> TickResult {
    let id = run.run_id;

    // Create child runs on first tick
    if run.children.is_empty() {
        if let Some(args) = &run.args {
            run.children = args.children.iter().map(|c| super::init_run(c)).collect();
        }
        info!("[#{id}] PARALLEL start ({} children)", run.children.len());
    }

    let child_count = run.children.len();
    let mut succeeded = 0u32;
    let mut failed = 0u32;

    for child in &mut run.children {
        match super::tick(api, child) {
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
