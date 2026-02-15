//! Takeoff node -- sends takeoff command and monitors altitude.

use log::info;
use prost::Message;

use crate::actions::io::ActionIO;
use crate::actions_dioxus::core::behavior::{ActionNode, Behavior, NodeResponse};
use crate::actions_dioxus::core::data::ProtoBytes;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.takeoff.rs"));
}
use proto::*;

const ALTITUDE_TOLERANCE_M: f64 = 1.0;

// ── Component ───────────────────────────────────────────────────

use dioxus::prelude::*;
#[allow(non_snake_case, unused)]
mod dioxus_elements {
    pub use crate::actions_dioxus::core::elements::*;
}

#[component]
pub fn Takeoff(altitude_m: f64) -> Element {
    let args = ProtoBytes(TakeoffArgs { altitude_m }.encode_to_vec());
    rsx! { takeoff { args } }
}

// ── Node ────────────────────────────────────────────────────────

pub type TakeoffNode = ActionNode<TakeoffArgs, TakeoffOutput, TakeoffResult, TakeoffState>;

impl Behavior for TakeoffNode {
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse {
        self.output.target_altitude_m = self.args.altitude_m;
        self.output.current_altitude_m = io.sense_status.altitude_m;
        info!("[takeoff] start: target={:.1}m", self.args.altitude_m);
        let _ = crate::controls::send_takeoff(io.cmd, self.args.altitude_m);
        NodeResponse::Running
    }

    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        self.output.current_altitude_m = io.sense_status.altitude_m;
        self.output.progress_pct = (self.output.current_altitude_m / self.args.altitude_m * 100.0).clamp(0.0, 100.0);
        self.state.current_altitude_m = self.output.current_altitude_m;

        if (self.output.current_altitude_m - self.args.altitude_m).abs() < ALTITUDE_TOLERANCE_M {
            self.result.reached_altitude_m = self.output.current_altitude_m;
            info!("[takeoff] reached {:.1}m", self.output.current_altitude_m);
            NodeResponse::Success
        } else {
            NodeResponse::Running
        }
    }
}
