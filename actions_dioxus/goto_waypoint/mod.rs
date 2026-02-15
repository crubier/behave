//! GotoWaypoint node -- sends goto command and monitors position.

use log::info;
use prost::Message;

use crate::actions::io::ActionIO;
use crate::actions_dioxus::core::behavior::{ActionNode, Behavior, NodeResponse};
use crate::actions_dioxus::core::data::ProtoBytes;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.goto.rs"));
}
use proto::*;

const ARRIVAL_TOLERANCE_M: f64 = 5.0;

// ── Component ───────────────────────────────────────────────────

use dioxus::prelude::*;
#[allow(non_snake_case, unused)]
mod dioxus_elements {
    pub use crate::actions_dioxus::core::elements::*;
}

#[component]
pub fn Goto(easting_m: f64, northing_m: f64, altitude_m: f64, speed_ms: f64) -> Element {
    let args = ProtoBytes(GotoArgs { easting_m, northing_m, altitude_m, speed_ms }.encode_to_vec());
    rsx! { goto { args } }
}

// ── Node ────────────────────────────────────────────────────────

pub type GotoWaypointNode = ActionNode<GotoArgs, GotoOutput, GotoResult, GotoState>;

impl Behavior for GotoWaypointNode {
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse {
        self.output.target_easting_m = self.args.easting_m;
        self.output.target_northing_m = self.args.northing_m;
        info!("[goto] start: target=({:.1}, {:.1})", self.args.easting_m, self.args.northing_m);
        let _ = crate::controls::send_goto(
            io.cmd, self.args.easting_m, self.args.northing_m,
            self.args.altitude_m, self.args.speed_ms,
        );
        NodeResponse::Running
    }

    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        self.output.current_easting_m = io.sense_status.easting_m;
        self.output.current_northing_m = io.sense_status.northing_m;
        let de = self.output.current_easting_m - self.args.easting_m;
        let dn = self.output.current_northing_m - self.args.northing_m;
        self.output.remaining_m = (de * de + dn * dn).sqrt();

        self.state.current_easting_m = self.output.current_easting_m;
        self.state.current_northing_m = self.output.current_northing_m;
        self.state.current_altitude_m = io.sense_status.altitude_m;
        self.state.remaining_m = self.output.remaining_m;

        if self.output.remaining_m < ARRIVAL_TOLERANCE_M {
            self.result.final_easting_m = self.output.current_easting_m;
            self.result.final_northing_m = self.output.current_northing_m;
            self.result.final_altitude_m = self.state.current_altitude_m;
            info!("[goto] arrived ({:.1}m from target)", self.output.remaining_m);
            NodeResponse::Success
        } else {
            NodeResponse::Running
        }
    }
}
