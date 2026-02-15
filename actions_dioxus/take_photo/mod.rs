//! TakePhoto node -- triggers camera and waits for ack.

use std::collections::HashMap;

use log::info;

use crate::actions::io::ActionIO;
use crate::actions_dioxus::core::behavior::{NodeBehavior, NodeResponse};
use crate::controls;

pub struct TakePhotoNode;

impl TakePhotoNode {
    pub fn from_attrs(_attrs: &HashMap<&'static str, f64>) -> Self {
        Self
    }
}

impl NodeBehavior for TakePhotoNode {
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse {
        info!("[take_photo] start");
        let _ = controls::send_trigger_camera(io.cmd, "");
        NodeResponse::Running
    }

    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        if io.control_response.success {
            info!("[take_photo] captured");
            NodeResponse::Success
        } else {
            info!("[take_photo] waiting for ack");
            NodeResponse::Running
        }
    }
}
