//! Takeoff action -- sends takeoff command and monitors altitude via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, ActionArgsKind, ActionResultKind, Tick};
use crate::controls;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.takeoff.rs"));
}

const ALTITUDE_TOLERANCE_M: f64 = 1.0;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    if let ActionArgsKind::Takeoff(args) = &node.kind {
        info!("[#{}] TAKEOFF start: target={:.1}m (current={:.1}m)", node.id, args.altitude_m, io.sense_status.altitude_m);
        let _ = controls::send_takeoff(io.cmd, args.altitude_m);
    }
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), ActionResultKind> {
    if let ActionArgsKind::Takeoff(args) = &node.kind {
        let current = io.sense_status.altitude_m;
        if (current - args.altitude_m).abs() < ALTITUDE_TOLERANCE_M {
            info!("[#{}] TAKEOFF reached {:.1}m", node.id, current);
            Tick::Success(ActionResultKind::Takeoff(proto::TakeoffResult {
                reached_altitude_m: current,
                success: true,
            }))
        } else {
            let progress = (current / args.altitude_m * 100.0).clamp(0.0, 100.0);
            info!("[#{}] TAKEOFF climbing {:.0}% (alt={:.1}m)", node.id, progress, current);
            Tick::Running(())
        }
    } else {
        Tick::Failure(ActionResultKind::Takeoff(proto::TakeoffResult {
            reached_altitude_m: 0.0,
            success: false,
        }))
    }
}
