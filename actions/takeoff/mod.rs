//! Takeoff action -- sends takeoff command and monitors altitude via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, ActionArgs, Tick};
use crate::controls;

const ALTITUDE_TOLERANCE_M: f64 = 1.0;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    if let ActionArgs::Takeoff { altitude_m } = &node.args {
        info!("[#{}] TAKEOFF start: target={:.1}m (current={:.1}m)", node.id, altitude_m, io.sense_status.altitude_m);
        let _ = controls::send_takeoff(io.cmd, *altitude_m);
    }
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), bool> {
    if let ActionArgs::Takeoff { altitude_m } = &node.args {
        let current = io.sense_status.altitude_m;
        if (current - altitude_m).abs() < ALTITUDE_TOLERANCE_M {
            info!("[#{}] TAKEOFF reached {:.1}m", node.id, current);
            Tick::Success(true)
        } else {
            let progress = (current / altitude_m * 100.0).clamp(0.0, 100.0);
            info!("[#{}] TAKEOFF climbing {:.0}% (alt={:.1}m)", node.id, progress, current);
            Tick::Running(())
        }
    } else {
        Tick::Failure(false)
    }
}
