//! TakePhoto action -- sends camera trigger and waits for ack.

use log::info;

use super::{ActionIO, ActionRun, ActionResult, TickResult, action_result};
use crate::controls;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.take_photo.rs"));
}

pub fn tick(run: &mut ActionRun, io: &ActionIO) -> TickResult {
    let id = run.run_id;

    if run.outputs.is_empty() {
        info!("[#{id}] TAKE PHOTO start");
        let _ = controls::send_trigger_camera(io.cmd, "");
    }

    if io.control_response.success {
        info!("[#{id}] TAKE PHOTO captured");
        run.result = Some(ActionResult { result: Some(action_result::Result::TakePhoto(
            proto::TakePhotoResult { success: true },
        ))});
        TickResult::Success
    } else {
        TickResult::Running
    }
}
