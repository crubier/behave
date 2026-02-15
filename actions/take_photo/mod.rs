//! TakePhoto action -- sends camera trigger and waits for ack via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, Tick};
use crate::controls;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    let args = node.args_msg.get_root_as_reader::<crate::schema::actions::take_photo_capnp::take_photo_args::Reader<'_>>().unwrap();
    let tag = args.get_tag().unwrap().to_str().unwrap_or("");

    info!("[#{}] TAKE PHOTO start: tag=\"{}\"", node.id, tag);
    let _ = controls::send_trigger_camera(io.cmd, tag);
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), bool> {
    // Succeed once we get a successful ack for any command
    // (in practice the ack correlation should use command IDs)
    if io.control_response.success {
        info!("[#{}] TAKE PHOTO captured", node.id);
        Tick::Success(true)
    } else {
        info!("[#{}] TAKE PHOTO waiting for ack", node.id);
        Tick::Running(())
    }
}
