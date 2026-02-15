//! Land action -- sends land command and monitors altitude.

use log::info;

use super::{ActionIO, ActionRun, ActionResult, TickResult, action_result};
use crate::controls;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.land.rs"));
}

const TOUCHDOWN_ALTITUDE_M: f64 = 0.5;

pub fn tick(run: &mut ActionRun, args: &proto::LandArgs, io: &ActionIO) -> TickResult {
    let id = run.run_id;

    if run.outputs.is_empty() {
        info!("[#{id}] LAND start: descent={:.1}m/s", args.descent_speed_ms);
        let _ = controls::send_land(io.cmd, args.descent_speed_ms);
    }

    if io.sense_status.altitude_m < TOUCHDOWN_ALTITUDE_M {
        info!("[#{id}] LAND touchdown (alt={:.2}m)", io.sense_status.altitude_m);
        run.result = Some(ActionResult { result: Some(action_result::Result::Land(
            proto::LandResult { success: true },
        ))});
        TickResult::Success
    } else {
        TickResult::Running
    }
}
