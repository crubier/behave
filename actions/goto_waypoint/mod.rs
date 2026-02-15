use log::info;

use super::Tick;
use crate::controls::{self, CmdPublisher};
use crate::schema::actions::goto_waypoint_capnp::{goto_waypoint_args, goto_waypoint_state, GotoWaypointPhase};

use super::ActionNode;

pub fn start(node: &mut ActionNode, cmd: &dyn CmdPublisher) {
    let args = node.args_msg.get_root_as_reader::<goto_waypoint_args::Reader<'_>>().unwrap();
    let lat = args.get_latitude_deg();
    let lon = args.get_longitude_deg();
    let alt = args.get_altitude_m();
    let spd = args.get_speed_ms();

    info!("[#{}] GOTO start: ({:.6}, {:.6}) alt={:.1}m spd={:.1}m/s", node.id, lat, lon, alt, spd);

    {
        let mut state = node.state_msg.init_root::<goto_waypoint_state::Builder<'_>>();
        state.set_current_latitude_deg(0.0);
        state.set_current_longitude_deg(0.0);
        state.set_current_altitude_m(0.0);
        state.set_remaining_distance_m(1000.0);
        state.set_phase(GotoWaypointPhase::EnRoute);
    }

    let _ = controls::send_goto(cmd, lat, lon, alt, spd);
}

pub fn tick(node: &mut ActionNode, _cmd: &dyn CmdPublisher) -> Tick<(), bool> {
    let remaining;
    {
        let state = node.state_msg.get_root_as_reader::<goto_waypoint_state::Reader<'_>>().unwrap();
        remaining = state.get_remaining_distance_m();
    }

    let new_remaining = (remaining - 200.0).max(0.0);

    if new_remaining <= 0.0 {
        {
            let mut state = node.state_msg.get_root::<goto_waypoint_state::Builder<'_>>().unwrap();
            state.set_remaining_distance_m(0.0);
            state.set_phase(GotoWaypointPhase::Arrived);
        }
        info!("[#{}] GOTO arrived", node.id);
        Tick::Success(true)
    } else {
        let progress = (1.0 - new_remaining / 1000.0) * 100.0;
        {
            let mut state = node.state_msg.get_root::<goto_waypoint_state::Builder<'_>>().unwrap();
            state.set_remaining_distance_m(new_remaining);
        }
        info!("[#{}] GOTO en route {:.0}% ({:.0}m left)", node.id, progress, new_remaining);
        Tick::Running(())
    }
}
