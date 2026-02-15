//! Land action -- sends land command and monitors altitude via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, ActionArgs, Tick};
use crate::controls;

const TOUCHDOWN_ALTITUDE_M: f64 = 0.5;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    if let ActionArgs::Land { descent_speed_ms } = &node.args {
        info!("[#{}] LAND start: descent={:.1}m/s (current alt={:.1}m)", node.id, descent_speed_ms, io.sense_status.altitude_m);
        let _ = controls::send_land(io.cmd, *descent_speed_ms);
    }
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), bool> {
    let current = io.sense_status.altitude_m;
    if current < TOUCHDOWN_ALTITUDE_M {
        info!("[#{}] LAND touchdown (alt={:.2}m)", node.id, current);
        Tick::Success(true)
    } else {
        info!("[#{}] LAND descending (alt={:.1}m)", node.id, current);
        Tick::Running(())
    }
}
