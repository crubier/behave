//! ReturnHome node -- sends return-home command and monitors mode.

use log::info;
use prost::Message;

use crate::actions::io::ActionIO;
use crate::actions_dioxus::core::behavior::{ActionNode, Behavior, NodeResponse};
use crate::actions_dioxus::core::data::ProtoBytes;
use crate::topics::control::status::MODE_RETURNING;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.home.rs"));
}
use proto::*;

// ── Component ───────────────────────────────────────────────────

use dioxus::prelude::*;
#[allow(non_snake_case, unused)]
mod dioxus_elements {
    pub use crate::actions_dioxus::core::elements::*;
}

#[component]
pub fn Home(altitude_m: f64) -> Element {
    let args = ProtoBytes(HomeArgs { altitude_m }.encode_to_vec());
    rsx! { home { args } }
}

// ── Node ────────────────────────────────────────────────────────

pub type ReturnHomeNode = ActionNode<HomeArgs, HomeOutput, HomeResult, HomeState>;

impl Behavior for ReturnHomeNode {
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse {
        info!("[home] start: alt={:.1}m", self.args.altitude_m);
        let _ = crate::controls::send_return_home(io.cmd, self.args.altitude_m);
        NodeResponse::Running
    }

    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        self.output.mode = io.control_status.mode as u32;
        self.state.mode = self.output.mode;

        if io.control_status.mode != MODE_RETURNING {
            self.result.final_mode = self.output.mode;
            info!("[home] arrived (mode={})", self.output.mode);
            NodeResponse::Success
        } else {
            NodeResponse::Running
        }
    }
}
