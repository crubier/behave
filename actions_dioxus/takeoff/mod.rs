//! Takeoff node -- sends takeoff command and monitors altitude.

use std::collections::HashMap;

use log::info;

use crate::actions::io::ActionIO;
use crate::actions_dioxus::core::behavior::{NodeBehavior, NodeResponse};
use crate::controls;

const ALTITUDE_TOLERANCE_M: f64 = 1.0;

pub struct TakeoffNode {
    altitude_m: f64,
}

impl TakeoffNode {
    pub fn from_attrs(attrs: &HashMap<&'static str, f64>) -> Self {
        Self {
            altitude_m: attrs.get("altitude_m").copied().unwrap_or(10.0),
        }
    }
}

impl NodeBehavior for TakeoffNode {
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse {
        info!(
            "[takeoff] start: target={:.1}m (current={:.1}m)",
            self.altitude_m, io.sense_status.altitude_m
        );
        let _ = controls::send_takeoff(io.cmd, self.altitude_m);
        NodeResponse::Running
    }

    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        let current = io.sense_status.altitude_m;
        if (current - self.altitude_m).abs() < ALTITUDE_TOLERANCE_M {
            info!("[takeoff] reached {:.1}m", current);
            NodeResponse::Success
        } else {
            let progress = (current / self.altitude_m * 100.0).clamp(0.0, 100.0);
            info!("[takeoff] climbing {:.0}% (alt={:.1}m)", progress, current);
            NodeResponse::Running
        }
    }
}
