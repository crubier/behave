//! GotoWaypoint action -- sends goto command and monitors position via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, ActionArgsKind, ActionResultKind, Tick};
use crate::controls;
use crate::schema::behave::actions::GotoWaypointResultArgs;

const ARRIVAL_TOLERANCE_M: f64 = 5.0;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    if let ActionArgsKind::GotoWaypoint(args) = &node.kind {
        info!("[#{}] GOTO start: target=({:.1}, {:.1}) alt={:.1}m spd={:.1}m/s",
            node.id, args.easting_m, args.northing_m, args.altitude_m, args.speed_ms);
        let _ = controls::send_goto(io.cmd, args.easting_m, args.northing_m, args.altitude_m, args.speed_ms);
    }
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), ActionResultKind> {
    if let ActionArgsKind::GotoWaypoint(args) = &node.kind {
        let de = io.sense_status.easting_m - args.easting_m;
        let dn = io.sense_status.northing_m - args.northing_m;
        let dist = (de * de + dn * dn).sqrt();
        if dist < ARRIVAL_TOLERANCE_M {
            info!("[#{}] GOTO arrived ({:.1}m from target)", node.id, dist);
            Tick::Success(ActionResultKind::GotoWaypoint(GotoWaypointResultArgs {
                final_easting_m: io.sense_status.easting_m,
                final_northing_m: io.sense_status.northing_m,
                final_altitude_m: io.sense_status.altitude_m,
                success: true,
            }))
        } else {
            info!("[#{}] GOTO en route ({:.0}m remaining)", node.id, dist);
            Tick::Running(())
        }
    } else {
        Tick::Failure(ActionResultKind::GotoWaypoint(GotoWaypointResultArgs {
            final_easting_m: 0.0,
            final_northing_m: 0.0,
            final_altitude_m: 0.0,
            success: false,
        }))
    }
}
