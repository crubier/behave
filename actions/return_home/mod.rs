//! ReturnHome action -- sends return-home command and monitors mode via ActionIO.

use log::info;

use super::{ActionIO, ActionNode, ActionArgsKind, ActionResultKind, Tick};
use crate::controls;
use crate::schema::behave::actions::ReturnHomeResultArgs;
use crate::topics::control::status::MODE_RETURNING;

pub fn start(node: &mut ActionNode, io: &ActionIO) {
    if let ActionArgsKind::ReturnHome(args) = &node.kind {
        info!("[#{}] RETURN HOME start: alt={:.1}m", node.id, args.altitude_m);
        let _ = controls::send_return_home(io.cmd, args.altitude_m);
    }
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), ActionResultKind> {
    let mode = io.control_status.mode;
    if mode != MODE_RETURNING {
        info!("[#{}] RETURN HOME arrived (mode={})", node.id, mode);
        Tick::Success(ActionResultKind::ReturnHome(ReturnHomeResultArgs { success: true }))
    } else {
        info!("[#{}] RETURN HOME en route", node.id);
        Tick::Running(())
    }
}
