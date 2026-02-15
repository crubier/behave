//! TakePhoto action -- sends camera trigger and waits for ack via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, ActionArgsKind, ActionResultKind, Tick};
use crate::controls;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.take_photo.rs"));
}

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    if let ActionArgsKind::TakePhoto(_) = &node.kind {
        info!("[#{}] TAKE PHOTO start", node.id);
        let _ = controls::send_trigger_camera(io.cmd, "");
    }
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), ActionResultKind> {
    if io.control_response.success {
        info!("[#{}] TAKE PHOTO captured", node.id);
        Tick::Success(ActionResultKind::TakePhoto(proto::TakePhotoResult { success: true }))
    } else {
        info!("[#{}] TAKE PHOTO waiting for ack", node.id);
        Tick::Running(())
    }
}
