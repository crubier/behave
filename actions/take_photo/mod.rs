use log::info;

use super::Tick;
use crate::controls::{self, CmdPublisher};
use crate::schema::actions::take_photo_capnp::{take_photo_args, take_photo_state, TakePhotoPhase};

use super::ActionNode;

pub fn start(node: &mut ActionNode, cmd: &dyn CmdPublisher) {
    let args = node.args_msg.get_root_as_reader::<take_photo_args::Reader<'_>>().unwrap();
    let tag = args.get_tag().unwrap().to_str().unwrap_or("");

    info!("[#{}] TAKE PHOTO start: tag=\"{}\"", node.id, tag);

    {
        let mut state = node.state_msg.init_root::<take_photo_state::Builder<'_>>();
        state.set_phase(TakePhotoPhase::Capturing);
    }

    let _ = controls::send_trigger_camera(cmd, tag);
}

pub fn tick(node: &mut ActionNode, _cmd: &dyn CmdPublisher) -> Tick<(), bool> {
    // Stub: photo captured instantly in one tick
    {
        let mut state = node.state_msg.get_root::<take_photo_state::Builder<'_>>().unwrap();
        state.set_phase(TakePhotoPhase::Done);
    }
    info!("[#{}] TAKE PHOTO captured", node.id);
    Tick::Success(true)
}
