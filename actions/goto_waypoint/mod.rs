//! GotoWaypoint action -- sends goto command and monitors position via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, ActionArgs, Tick};
use crate::controls;

const ARRIVAL_TOLERANCE_M: f64 = 5.0;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    if let ActionArgs::GotoWaypoint { easting_m, northing_m, altitude_m, speed_ms } = &node.args {
        info!("[#{}] GOTO start: target=({:.1}, {:.1}) alt={:.1}m spd={:.1}m/s", node.id, easting_m, northing_m, altitude_m, speed_ms);
        let _ = controls::send_goto(io.cmd, *easting_m, *northing_m, *altitude_m, *speed_ms);
    }
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), bool> {
    if let ActionArgs::GotoWaypoint { easting_m, northing_m, .. } = &node.args {
        let de = io.sense_status.easting_m - easting_m;
        let dn = io.sense_status.northing_m - northing_m;
        let dist = (de * de + dn * dn).sqrt();

        if dist < ARRIVAL_TOLERANCE_M {
            info!("[#{}] GOTO arrived ({:.1}m from target)", node.id, dist);
            Tick::Success(true)
        } else {
            info!("[#{}] GOTO en route ({:.0}m remaining)", node.id, dist);
            Tick::Running(())
        }
    } else {
        Tick::Failure(false)
    }
}
