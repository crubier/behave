//! Land action -- sends land command and monitors altitude via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, ActionArgsKind, ActionResultKind, Tick};
use crate::controls;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.land.rs"));
}

const TOUCHDOWN_ALTITUDE_M: f64 = 0.5;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    if let ActionArgsKind::Land(args) = &node.kind {
        info!("[#{}] LAND start: descent={:.1}m/s (current alt={:.1}m)", node.id, args.descent_speed_ms, io.sense_status.altitude_m);
        let _ = controls::send_land(io.cmd, args.descent_speed_ms);
    }
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), ActionResultKind> {
    let current = io.sense_status.altitude_m;
    if current < TOUCHDOWN_ALTITUDE_M {
        info!("[#{}] LAND touchdown (alt={:.2}m)", node.id, current);
        Tick::Success(ActionResultKind::Land(proto::LandResult { success: true }))
    } else {
        info!("[#{}] LAND descending (alt={:.1}m)", node.id, current);
        Tick::Running(())
    }
}
