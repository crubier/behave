//! ReturnHome action -- sends return-home command and monitors mode via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, ActionArgs, Tick};
use crate::topics::control::status::MODE_RETURNING;
use crate::controls;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    if let ActionArgs::ReturnHome { altitude_m } = &node.args {
        info!("[#{}] RETURN HOME start: alt={:.1}m", node.id, altitude_m);
        let _ = controls::send_return_home(io.cmd, *altitude_m);
    }
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
