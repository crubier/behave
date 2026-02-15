//! ReturnHome action -- sends return-home command and monitors mode.

use log::info;

use super::{ActionIO, ActionRun, ActionResult, TickResult, action_result};
use crate::controls;
use crate::topics::control::status::MODE_RETURNING;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.return_home.rs"));
}

pub fn tick(run: &mut ActionRun, args: &proto::ReturnHomeArgs, io: &ActionIO) -> TickResult {
    let id = run.run_id;

    if run.outputs.is_empty() {
        info!("[#{id}] RETURN HOME start: alt={:.1}m", args.altitude_m);
        let _ = controls::send_return_home(io.cmd, args.altitude_m);
    }

    if io.control_status.mode != MODE_RETURNING {
        info!("[#{id}] RETURN HOME arrived (mode={})", io.control_status.mode);
        run.result = Some(ActionResult { result: Some(action_result::Result::ReturnHome(
            proto::ReturnHomeResult { success: true },
        ))});
        TickResult::Success
    } else {
        TickResult::Running
    }
}
