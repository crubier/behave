//! TakePhoto node -- triggers camera and waits for ack.

use log::info;
use prost::Message;

use crate::actions::io::ActionIO;
use crate::actions_dioxus::core::behavior::{ActionNode, Behavior, NodeResponse};
use crate::actions_dioxus::core::data::ProtoBytes;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.photo.rs"));
}
use proto::*;

// ── Component ───────────────────────────────────────────────────

use dioxus::prelude::*;
#[allow(non_snake_case, unused)]
mod dioxus_elements {
    pub use crate::actions_dioxus::core::elements::*;
}

#[component]
pub fn Photo() -> Element {
    let args = ProtoBytes(PhotoArgs {}.encode_to_vec());
    rsx! { photo { args } }
}

// ── Node ────────────────────────────────────────────────────────

pub type TakePhotoNode = ActionNode<PhotoArgs, PhotoOutput, PhotoResult, PhotoState>;

impl Behavior for TakePhotoNode {
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse {
        info!("[photo] start");
        let _ = crate::controls::send_trigger_camera(io.cmd, "");
        NodeResponse::Running
    }

    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        if io.control_response.success {
            self.output.acked = true;
            self.result.captured = true;
            self.state.acked = true;
            info!("[photo] captured");
            NodeResponse::Success
        } else {
            NodeResponse::Running
        }
    }
}
