//! GotoWaypoint action -- sends goto command and monitors position.

use log::info;

use super::{ActionIO, ActionRun, ActionResult, TickResult, action_result};
use crate::controls;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.goto_waypoint.rs"));
}

const ARRIVAL_TOLERANCE_M: f64 = 5.0;

pub fn tick(run: &mut ActionRun, args: &proto::GotoWaypointArgs, io: &ActionIO) -> TickResult {
    let id = run.run_id;

    if run.outputs.is_empty() {
        info!("[#{id}] GOTO start: target=({:.1}, {:.1}) alt={:.1}m spd={:.1}m/s",
            args.easting_m, args.northing_m, args.altitude_m, args.speed_ms);
        let _ = controls::send_goto(io.cmd, args.easting_m, args.northing_m, args.altitude_m, args.speed_ms);
    }

    let de = io.sense_status.easting_m - args.easting_m;
    let dn = io.sense_status.northing_m - args.northing_m;
    let dist = (de * de + dn * dn).sqrt();

    if dist < ARRIVAL_TOLERANCE_M {
        info!("[#{id}] GOTO arrived ({:.1}m from target)", dist);
        run.result = Some(ActionResult { result: Some(action_result::Result::GotoWaypoint(
            proto::GotoWaypointResult {
                final_easting_m: io.sense_status.easting_m,
                final_northing_m: io.sense_status.northing_m,
                final_altitude_m: io.sense_status.altitude_m,
                success: true,
            },
        ))});
        TickResult::Success
    } else {
        info!("[#{id}] GOTO en route ({:.0}m remaining)", dist);
        TickResult::Running
    }
}
