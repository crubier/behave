//! TakePhoto action -- sends camera trigger and waits for ack via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, ActionArgs, Tick};
use crate::controls;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    if let ActionArgs::TakePhoto { tag } = &node.args {
        info!("[#{}] TAKE PHOTO start: tag=\"{}\"", node.id, tag);
        let _ = controls::send_trigger_camera(io.cmd, tag);
    }
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), bool> {
    if io.control_response.success {
        info!("[#{}] TAKE PHOTO captured", node.id);
        Tick::Success(true)
    } else {
        info!("[#{}] TAKE PHOTO waiting for ack", node.id);
        Tick::Running(())
    }
}
