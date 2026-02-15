//! Takeoff action -- sends takeoff command and monitors altitude.

use log::info;

use super::{ActionIO, ActionRun, ActionResult, TickResult, action_result};
use crate::controls;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.takeoff.rs"));
}

const ALTITUDE_TOLERANCE_M: f64 = 1.0;

pub fn tick(run: &mut ActionRun, args: &proto::TakeoffArgs, io: &ActionIO) -> TickResult {
    let id = run.run_id;

    // First tick: send command
    if run.started_at == super::now_utime() || run.result.is_none() && run.outputs.is_empty() {
        info!("[#{id}] TAKEOFF start: target={:.1}m", args.altitude_m);
        let _ = controls::send_takeoff(io.cmd, args.altitude_m);
    }

    let current = io.sense_status.altitude_m;
    if (current - args.altitude_m).abs() < ALTITUDE_TOLERANCE_M {
        info!("[#{id}] TAKEOFF reached {:.1}m", current);
        run.result = Some(ActionResult { result: Some(action_result::Result::Takeoff(
            proto::TakeoffResult { reached_altitude_m: current, success: true },
        ))});
        TickResult::Success
    } else {
        TickResult::Running
    }
}
