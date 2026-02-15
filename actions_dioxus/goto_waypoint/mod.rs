//! GotoWaypoint node -- sends goto command and monitors position.

use std::collections::HashMap;

use log::info;

use crate::actions::io::ActionIO;
use crate::actions_dioxus::core::behavior::{NodeBehavior, NodeResponse};
use crate::controls;

const ARRIVAL_TOLERANCE_M: f64 = 5.0;

pub struct GotoWaypointNode {
    easting_m: f64,
    northing_m: f64,
    altitude_m: f64,
    speed_ms: f64,
}

impl GotoWaypointNode {
    pub fn from_attrs(attrs: &HashMap<&'static str, f64>) -> Self {
        Self {
            easting_m: attrs.get("easting_m").copied().unwrap_or(0.0),
            northing_m: attrs.get("northing_m").copied().unwrap_or(0.0),
            altitude_m: attrs.get("altitude_m").copied().unwrap_or(50.0),
            speed_ms: attrs.get("speed_ms").copied().unwrap_or(10.0),
        }
    }
}

impl NodeBehavior for GotoWaypointNode {
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse {
        info!(
            "[goto_waypoint] start: target=({:.1}, {:.1}) alt={:.1}m spd={:.1}m/s",
            self.easting_m, self.northing_m, self.altitude_m, self.speed_ms
        );
        let _ = controls::send_goto(
            io.cmd,
            self.easting_m,
            self.northing_m,
            self.altitude_m,
            self.speed_ms,
        );
        NodeResponse::Running
    }

    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        let de = io.sense_status.easting_m - self.easting_m;
        let dn = io.sense_status.northing_m - self.northing_m;
        let dist = (de * de + dn * dn).sqrt();
        if dist < ARRIVAL_TOLERANCE_M {
            info!("[goto_waypoint] arrived ({:.1}m from target)", dist);
            NodeResponse::Success
        } else {
            info!("[goto_waypoint] en route ({:.0}m remaining)", dist);
            NodeResponse::Running
        }
    }
}
