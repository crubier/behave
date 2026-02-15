use log::info;

use super::Tick;
use crate::controls::CmdPublisher;
use crate::schema::actions::sequence_capnp::sequence_state;

use super::ActionNode;

pub fn start(node: &mut ActionNode) {
    let child_count = node.children.len();
    info!("[#{}] SEQUENCE \"{}\" start ({} children)", node.id, node.name, child_count);

    {
        let mut state = node.state_msg.init_root::<sequence_state::Builder<'_>>();
        state.set_current_index(0);
        // Note: child_states list is tracked via node.children, not in capnp state
    }
}

pub fn tick(node: &mut ActionNode, cmd: &dyn CmdPublisher) -> Tick<(), bool> {
    let current_index;
    {
        let state = node.state_msg.get_root_as_reader::<sequence_state::Reader<'_>>().unwrap();
        current_index = state.get_current_index() as usize;
    }

    let child_count = node.children.len();

    if current_index >= child_count {
        return Tick::Success(true);
    }

    // Tick the current child (start_node is called lazily inside super::tick)
    let child_result = super::tick(&mut node.children[current_index], cmd);

    match child_result {
        Tick::Running(()) => {
            let progress = current_index as f64 / child_count as f64 * 100.0;
            info!("[#{}] SEQUENCE \"{}\" running child {}/{} ({:.0}%)",
                node.id, node.name, current_index, child_count, progress);
            Tick::Running(())
        }
        Tick::Success(_) => {
            let next = current_index + 1;
            info!("[#{}] SEQUENCE \"{}\" child {} succeeded ({}/{})",
                node.id, node.name, current_index, next, child_count);

            {
                let mut state = node.state_msg.get_root::<sequence_state::Builder<'_>>().unwrap();
                state.set_current_index(next as u32);
            }

            if next >= child_count {
                info!("[#{}] SEQUENCE \"{}\" all children done", node.id, node.name);
                Tick::Success(true)
            } else {
                Tick::Running(())
            }
        }
        Tick::Failure(_) => {
            info!("[#{}] SEQUENCE \"{}\" child {} FAILED -- aborting", node.id, node.name, current_index);
            Tick::Failure(false)
        }
    }
}
