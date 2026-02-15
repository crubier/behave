//! Loop composite -- repeats its single child.
//! max_iterations=0 means infinite. Fails if any iteration fails.

use log::info;

use super::{ActionIO, ActionRun, ActionResult, TickResult, action_result};

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.loop_action.rs"));
}

pub fn tick(run: &mut ActionRun, args: &proto::LoopArgs, io: &ActionIO) -> TickResult {
    let id = run.run_id;
    let max = args.max_iterations;

    if run.children.is_empty() {
        run.result = Some(ActionResult { result: Some(action_result::Result::Loop(
            proto::LoopResult { success: true, iterations_completed: 0 },
        ))});
        return TickResult::Success;
    }

    let child = &mut run.children[0];
    let child_result = super::tick(child, io);

    match child_result {
        TickResult::Running => TickResult::Running,
        TickResult::Success => {
            // Count completed iterations from children runs
            let iteration = run.children.iter()
                .filter(|c| c.status == super::RunStatus::Succeeded as i32)
                .count();
            info!("[#{id}] LOOP iteration {iteration} completed");

            if max > 0 && iteration >= max as usize {
                info!("[#{id}] LOOP done ({iteration} iterations)");
                run.result = Some(ActionResult { result: Some(action_result::Result::Loop(
                    proto::LoopResult { success: true, iterations_completed: iteration as u32 },
                ))});
                TickResult::Success
            } else {
                // Reset child for next iteration by re-initializing it
                if let Some(child_args) = run.args.as_ref().and_then(|a| a.children.first()) {
                    run.children[0] = super::init_run(child_args);
                }
                TickResult::Running
            }
        }
        TickResult::Failure => {
            let iteration = run.children.iter()
                .filter(|c| c.status == super::RunStatus::Succeeded as i32)
                .count();
            info!("[#{id}] LOOP child FAILED at iteration {}", iteration + 1);
            run.result = Some(ActionResult { result: Some(action_result::Result::Loop(
                proto::LoopResult { success: false, iterations_completed: iteration as u32 },
            ))});
            TickResult::Failure
        }
    }
}
