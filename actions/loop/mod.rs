//! Loop composite -- repeats its single child.
//! max_iterations=0 means infinite. Fails if any iteration fails.
//!
//! Each iteration creates a new ActionRun in `children`.
//! The last child is the current (active) iteration.

use log::info;

use super::{ActionAPI, ActionRun, ActionResult, TickResult, action_result, get_args};

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.loop_action.rs"));
}

pub fn tick(api: &ActionAPI, run: &mut ActionRun) -> TickResult {
    let args = get_args!(run, Loop);
    let id = run.run_id;
    let max = args.max_iterations;

    let child_args = match run.args.as_ref().and_then(|a| a.children.first()) {
        Some(a) => a.clone(),
        None => {
            run.result = Some(ActionResult { result: Some(action_result::Result::Loop(
                proto::LoopResult { success: true, iterations_completed: 0 },
            ))});
            return TickResult::Success;
        }
    };

    // If no active iteration yet, or last iteration completed, start a new one
    let needs_new = run.children.is_empty()
        || run.children.last().map(|c| c.status == super::RunStatus::Succeeded as i32).unwrap_or(false);

    if needs_new {
        let completed = run.children.iter()
            .filter(|c| c.status == super::RunStatus::Succeeded as i32)
            .count();

        // Check if we've hit max
        if max > 0 && completed >= max as usize {
            info!("[#{id}] LOOP done ({completed} iterations)");
            run.result = Some(ActionResult { result: Some(action_result::Result::Loop(
                proto::LoopResult { success: true, iterations_completed: completed as u32 },
            ))});
            return TickResult::Success;
        }

        // Start new iteration
        info!("[#{id}] LOOP starting iteration {}", completed + 1);
        run.children.push(super::init_run(&child_args));
    }

    // Tick the current (last) iteration
    let child_result = super::tick(api, run.children.last_mut().unwrap());

    match child_result {
        TickResult::Running => TickResult::Running,
        TickResult::Success => {
            let completed = run.children.iter()
                .filter(|c| c.status == super::RunStatus::Succeeded as i32)
                .count();
            info!("[#{id}] LOOP iteration {completed} completed");

            if max > 0 && completed >= max as usize {
                info!("[#{id}] LOOP done ({completed} iterations)");
                run.result = Some(ActionResult { result: Some(action_result::Result::Loop(
                    proto::LoopResult { success: true, iterations_completed: completed as u32 },
                ))});
                TickResult::Success
            } else {
                TickResult::Running
            }
        }
        TickResult::Failure => {
            let completed = run.children.iter()
                .filter(|c| c.status == super::RunStatus::Succeeded as i32)
                .count();
            info!("[#{id}] LOOP child FAILED at iteration {}", completed + 1);
            run.result = Some(ActionResult { result: Some(action_result::Result::Loop(
                proto::LoopResult { success: false, iterations_completed: completed as u32 },
            ))});
            TickResult::Failure
        }
    }
}
