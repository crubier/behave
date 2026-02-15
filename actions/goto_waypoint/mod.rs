//! GotoWaypoint action -- sends goto command and monitors position via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, Tick};
use crate::controls;

const ARRIVAL_TOLERANCE_M: f64 = 5.0;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    let args = node.args_msg.get_root_as_reader::<crate::schema::actions::goto_waypoint_capnp::goto_waypoint_args::Reader<'_>>().unwrap();
    let east = args.get_easting_m();
    let north = args.get_northing_m();
    let alt = args.get_altitude_m();
    let spd = args.get_speed_ms();

    info!("[#{}] GOTO start: target=({:.1}, {:.1}) alt={:.1}m spd={:.1}m/s", node.id, east, north, alt, spd);
    let _ = controls::send_goto(io.cmd, east, north, alt, spd);
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), bool> {
    let args = node.args_msg.get_root_as_reader::<crate::schema::actions::goto_waypoint_capnp::goto_waypoint_args::Reader<'_>>().unwrap();
    let target_e = args.get_easting_m();
    let target_n = args.get_northing_m();

    let de = io.sense_status.easting_m - target_e;
    let dn = io.sense_status.northing_m - target_n;
    let dist = (de * de + dn * dn).sqrt();

    if dist < ARRIVAL_TOLERANCE_M {
        info!("[#{}] GOTO arrived ({:.1}m from target)", node.id, dist);
        Tick::Success(true)
    } else {
        info!("[#{}] GOTO en route ({:.0}m remaining)", node.id, dist);
        Tick::Running(())
    }
}
