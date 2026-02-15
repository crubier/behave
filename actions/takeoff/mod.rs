//! Takeoff action -- sends takeoff command and monitors altitude via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, Tick};
use crate::controls;

const ALTITUDE_TOLERANCE_M: f64 = 1.0;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    let args = node.args_msg.get_root_as_reader::<crate::schema::actions::takeoff_capnp::takeoff_args::Reader<'_>>().unwrap();
    let alt = args.get_altitude_m();

    info!("[#{}] TAKEOFF start: target={:.1}m (current={:.1}m)", node.id, alt, io.sense_status.altitude_m);
    let _ = controls::send_takeoff(io.cmd, alt);
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), bool> {
    let args = node.args_msg.get_root_as_reader::<crate::schema::actions::takeoff_capnp::takeoff_args::Reader<'_>>().unwrap();
    let target = args.get_altitude_m();
    let current = io.sense_status.altitude_m;

    if (current - target).abs() < ALTITUDE_TOLERANCE_M {
        info!("[#{}] TAKEOFF reached {:.1}m", node.id, current);
        Tick::Success(true)
    } else {
        let progress = (current / target * 100.0).clamp(0.0, 100.0);
        info!("[#{}] TAKEOFF climbing {:.0}% (alt={:.1}m)", node.id, progress, current);
        Tick::Running(())
    }
}
