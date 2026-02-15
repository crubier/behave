use log::info;

use super::Tick;
use crate::controls::{self, CmdPublisher};
use crate::schema::actions::return_home_capnp::{return_home_args, return_home_state, ReturnHomePhase};

use super::ActionNode;

pub fn start(node: &mut ActionNode, cmd: &dyn CmdPublisher) {
    let args = node.args_msg.get_root_as_reader::<return_home_args::Reader<'_>>().unwrap();
    let alt = args.get_altitude_m();

    info!("[#{}] RETURN HOME start: alt={:.1}m", node.id, alt);

    {
        let mut state = node.state_msg.init_root::<return_home_state::Builder<'_>>();
        state.set_remaining_distance_m(500.0);
        state.set_phase(ReturnHomePhase::EnRoute);
    }

    let _ = controls::send_return_home(cmd, alt);
}

pub fn tick(node: &mut ActionNode, _cmd: &dyn CmdPublisher) -> Tick<(), bool> {
    let remaining;
    {
        let state = node.state_msg.get_root_as_reader::<return_home_state::Reader<'_>>().unwrap();
        remaining = state.get_remaining_distance_m();
    }

    let new_remaining = (remaining - 200.0).max(0.0);

    if new_remaining <= 0.0 {
        {
            let mut state = node.state_msg.get_root::<return_home_state::Builder<'_>>().unwrap();
            state.set_remaining_distance_m(0.0);
            state.set_phase(ReturnHomePhase::Arrived);
        }
        info!("[#{}] RETURN HOME arrived", node.id);
        Tick::Success(true)
    } else {
        let progress = (1.0 - new_remaining / 500.0) * 100.0;
        {
            let mut state = node.state_msg.get_root::<return_home_state::Builder<'_>>().unwrap();
            state.set_remaining_distance_m(new_remaining);
        }
        info!("[#{}] RETURN HOME en route {:.0}%", node.id, progress);
        Tick::Running(())
    }
}
