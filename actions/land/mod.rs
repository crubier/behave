//! Land action -- sends land command and monitors altitude.

use log::info;

use super::{ActionAPI, ActionRun, ActionResult, TickResult, action_result, get_args};
use crate::controls;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.land.rs"));
}

const TOUCHDOWN_ALTITUDE_M: f64 = 0.5;

pub fn tick(api: &ActionAPI, run: &mut ActionRun) -> TickResult {
    let args = get_args!(run, Land);
    let id = run.run_id;

    if run.outputs.is_empty() {
        info!("[#{id}] LAND start: descent={:.1}m/s", args.descent_speed_ms);
        let _ = controls::send_land(api.cmd, args.descent_speed_ms);
    }

    if api.sense_status.altitude_m < TOUCHDOWN_ALTITUDE_M {
        info!("[#{id}] LAND touchdown (alt={:.2}m)", api.sense_status.altitude_m);
        run.result = Some(ActionResult { result: Some(action_result::Result::Land(
            proto::LandResult { success: true },
        ))});
        TickResult::Success
    } else {
        TickResult::Running
    }
}
