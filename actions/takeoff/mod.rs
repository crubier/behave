use log::info;

use crate::action::Tick;
use crate::controls::{self, CmdPublisher};
use crate::schema::actions::takeoff_capnp::{takeoff_args, takeoff_state, TakeoffPhase};

use super::ActionNode;

pub fn start(node: &mut ActionNode, cmd: &dyn CmdPublisher) {
    let args = node.args_msg.get_root_as_reader::<takeoff_args::Reader<'_>>().unwrap();
    let alt = args.get_altitude_m();

    info!("[#{}] TAKEOFF start: alt={:.1}m", node.id, alt);

    {
        let mut state = node.state_msg.init_root::<takeoff_state::Builder<'_>>();
        state.set_current_altitude_m(0.0);
        state.set_phase(TakeoffPhase::Climbing);
    }

    let _ = controls::send_takeoff(cmd, alt);
}

pub fn tick(node: &mut ActionNode, _cmd: &dyn CmdPublisher) -> Tick<(), bool> {
    let args = node.args_msg.get_root_as_reader::<takeoff_args::Reader<'_>>().unwrap();
    let target = args.get_altitude_m();

    // Read current state
    let current_alt;
    {
        let state = node.state_msg.get_root_as_reader::<takeoff_state::Reader<'_>>().unwrap();
        current_alt = state.get_current_altitude_m();
    }

    let new_alt = current_alt + 5.0;

    if new_alt >= target {
        // Update state to reached
        {
            let mut state = node.state_msg.get_root::<takeoff_state::Builder<'_>>().unwrap();
            state.set_current_altitude_m(target);
            state.set_phase(TakeoffPhase::Reached);
        }
        info!("[#{}] TAKEOFF reached {:.1}m", node.id, target);
        Tick::Success(true)
    } else {
        let progress = (new_alt / target * 100.0).min(100.0);
        {
            let mut state = node.state_msg.get_root::<takeoff_state::Builder<'_>>().unwrap();
            state.set_current_altitude_m(new_alt);
        }
        info!("[#{}] TAKEOFF climbing {:.0}%", node.id, progress);
        Tick::Running(())
    }
}
