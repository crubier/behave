//! Land node -- sends land command and monitors altitude.

use log::info;
use prost::Message;

use crate::actions_dioxus::ActionIO;
use crate::actions_dioxus::core::behavior::{ActionNode, Behavior, NodeResponse};
use crate::actions_dioxus::core::data::ProtoBytes;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.land.rs"));
}
use proto::*;

const TOUCHDOWN_ALTITUDE_M: f64 = 0.5;

// ── Component ───────────────────────────────────────────────────

use dioxus::prelude::*;
#[allow(non_snake_case, unused)]
mod dioxus_elements {
    pub use crate::actions_dioxus::core::elements::*;
}

#[component]
pub fn Land(descent_speed_ms: f64) -> Element {
    let args = ProtoBytes(LandArgs { descent_speed_ms }.encode_to_vec());
    rsx! { land { args } }
}

// ── Node ────────────────────────────────────────────────────────

pub type LandNode = ActionNode<LandArgs, LandOutput, LandResult, LandInput, LandState>;

impl Behavior for LandNode {
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse {
        self.output.current_altitude_m = io.sense_status.altitude_m;
        info!("[land] start: descent={:.1}m/s", self.args.descent_speed_ms);
        let _ = crate::controls::send_land(io.cmd, self.args.descent_speed_ms);
        NodeResponse::Running
    }

    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        self.output.current_altitude_m = io.sense_status.altitude_m;
        self.state.current_altitude_m = self.output.current_altitude_m;

        if self.output.current_altitude_m < TOUCHDOWN_ALTITUDE_M {
            self.result.final_altitude_m = self.output.current_altitude_m;
            info!("[land] touchdown (alt={:.2}m)", self.output.current_altitude_m);
            NodeResponse::Success
        } else {
            NodeResponse::Running
        }
    }
}
