//! ReturnHome action -- sends return-home command and monitors mode via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, Tick};
use super::io::MODE_RETURNING;
use crate::controls;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    let args = node.args_msg.get_root_as_reader::<crate::schema::actions::return_home_capnp::return_home_args::Reader<'_>>().unwrap();
    let alt = args.get_altitude_m();

    info!("[#{}] RETURN HOME start: alt={:.1}m", node.id, alt);
    let _ = controls::send_return_home(io.cmd, alt);
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), bool> {
    let mode = io.control_status.mode;

    if mode != MODE_RETURNING {
        info!("[#{}] RETURN HOME arrived (mode={})", node.id, mode);
        Tick::Success(true)
    } else {
        info!("[#{}] RETURN HOME en route", node.id);
        Tick::Running(())
    }
}
