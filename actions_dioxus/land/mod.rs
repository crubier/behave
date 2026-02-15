//! Land node -- sends land command and monitors altitude.

use std::collections::HashMap;

use log::info;

use crate::actions::io::ActionIO;
use crate::actions_dioxus::core::behavior::{NodeBehavior, NodeResponse};
use crate::controls;

const TOUCHDOWN_ALTITUDE_M: f64 = 0.5;

pub struct LandNode {
    descent_speed_ms: f64,
}

impl LandNode {
    pub fn from_attrs(attrs: &HashMap<&'static str, f64>) -> Self {
        Self {
            descent_speed_ms: attrs.get("descent_speed_ms").copied().unwrap_or(2.0),
        }
    }
}

impl NodeBehavior for LandNode {
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse {
        info!(
            "[land] start: descent={:.1}m/s (current alt={:.1}m)",
            self.descent_speed_ms, io.sense_status.altitude_m
        );
        let _ = controls::send_land(io.cmd, self.descent_speed_ms);
        NodeResponse::Running
    }

    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        let current = io.sense_status.altitude_m;
        if current < TOUCHDOWN_ALTITUDE_M {
            info!("[land] touchdown (alt={:.2}m)", current);
            NodeResponse::Success
        } else {
            info!("[land] descending (alt={:.1}m)", current);
            NodeResponse::Running
        }
    }
}
