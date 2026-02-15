//! ReturnHome node -- sends return-home command and monitors mode.

use std::collections::HashMap;

use log::info;

use crate::actions::io::ActionIO;
use crate::actions_dioxus::core::behavior::{NodeBehavior, NodeResponse};
use crate::controls;
use crate::topics::control::status::MODE_RETURNING;

pub struct ReturnHomeNode {
    altitude_m: f64,
}

impl ReturnHomeNode {
    pub fn from_attrs(attrs: &HashMap<&'static str, f64>) -> Self {
        Self {
            altitude_m: attrs.get("altitude_m").copied().unwrap_or(50.0),
        }
    }
}

impl NodeBehavior for ReturnHomeNode {
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse {
        info!("[return_home] start: alt={:.1}m", self.altitude_m);
        let _ = controls::send_return_home(io.cmd, self.altitude_m);
        NodeResponse::Running
    }

    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        let mode = io.control_status.mode;
        if mode != MODE_RETURNING {
            info!("[return_home] arrived (mode={})", mode);
            NodeResponse::Success
        } else {
            info!("[return_home] en route");
            NodeResponse::Running
        }
    }
}
