use log::info;

use super::Tick;
use crate::controls::{self, CmdPublisher};
use crate::schema::actions::land_capnp::{land_args, land_state, LandPhase};

use super::ActionNode;

pub fn start(node: &mut ActionNode, cmd: &dyn CmdPublisher) {
    let args = node.args_msg.get_root_as_reader::<land_args::Reader<'_>>().unwrap();
    let spd = args.get_descent_speed_ms();

    info!("[#{}] LAND start: descent={:.1}m/s", node.id, spd);

    {
        let mut state = node.state_msg.init_root::<land_state::Builder<'_>>();
        state.set_current_altitude_m(50.0); // stub: assume some altitude
        state.set_phase(LandPhase::Descending);
    }

    let _ = controls::send_land(cmd, spd);
}

pub fn tick(node: &mut ActionNode, _cmd: &dyn CmdPublisher) -> Tick<(), bool> {
    let current_alt;
    {
        let state = node.state_msg.get_root_as_reader::<land_state::Reader<'_>>().unwrap();
        current_alt = state.get_current_altitude_m();
    }

    let new_alt = (current_alt - 10.0).max(0.0);

    if new_alt <= 0.0 {
        {
            let mut state = node.state_msg.get_root::<land_state::Builder<'_>>().unwrap();
            state.set_current_altitude_m(0.0);
            state.set_phase(LandPhase::Touchdown);
        }
        info!("[#{}] LAND touchdown", node.id);
        Tick::Success(true)
    } else {
        let progress = (1.0 - new_alt / 50.0) * 100.0;
        {
            let mut state = node.state_msg.get_root::<land_state::Builder<'_>>().unwrap();
            state.set_current_altitude_m(new_alt);
        }
        info!("[#{}] LAND descending {:.0}% (alt={:.1}m)", node.id, progress, new_alt);
        Tick::Running(())
    }
}
