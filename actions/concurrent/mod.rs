//! Concurrent composite -- ticks ALL children each cycle.
//! Succeeds when success_threshold children succeed (0 = all).

use log::info;

use super::{ActionIO, ActionRun, ActionResult, TickResult, action_result};

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.concurrent.rs"));
}

pub fn tick(run: &mut ActionRun, args: &proto::ConcurrentArgs, io: &ActionIO) -> TickResult {
    let id = run.run_id;

    // Create child runs on first tick
    if run.children.is_empty() {
        if let Some(run_args) = &run.args {
            run.children = run_args.children.iter().map(|c| super::init_run(c)).collect();
        }
        info!("[#{id}] CONCURRENT start ({} children)", run.children.len());
    }

    let child_count = run.children.len() as u32;
    let threshold = if args.success_threshold == 0 { child_count } else { args.success_threshold };

    let mut succeeded = 0u32;
    let mut failed = 0u32;

    for child in &mut run.children {
        match super::tick(child, io) {
            TickResult::Running => {}
            TickResult::Success => succeeded += 1,
            TickResult::Failure => failed += 1,
        }
    }

    if succeeded >= threshold {
        info!("[#{id}] CONCURRENT threshold reached ({succeeded}/{threshold})");
        run.result = Some(ActionResult { result: Some(action_result::Result::Concurrent(
            proto::ConcurrentResult { success: true, succeeded_count: succeeded, failed_count: failed, child_count },
        ))});
        TickResult::Success
    } else if child_count - failed < threshold {
        info!("[#{id}] CONCURRENT cannot reach threshold ({failed} failed, need {threshold})");
        run.result = Some(ActionResult { result: Some(action_result::Result::Concurrent(
            proto::ConcurrentResult { success: false, succeeded_count: succeeded, failed_count: failed, child_count },
        ))});
        TickResult::Failure
    } else {
        TickResult::Running
    }
}
