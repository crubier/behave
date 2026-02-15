use log::info;

use super::Tick;
use crate::controls::CmdPublisher;
use crate::schema::actions::fallback_capnp::fallback_state;

use super::ActionNode;

pub fn start(node: &mut ActionNode) {
    let child_count = node.children.len();
    info!("[#{}] FALLBACK \"{}\" start ({} children)", node.id, node.name, child_count);

    {
        let mut state = node.state_msg.init_root::<fallback_state::Builder<'_>>();
        state.set_current_index(0);
    }
}

pub fn tick(node: &mut ActionNode, cmd: &dyn CmdPublisher) -> Tick<(), bool> {
    let current_index;
    {
        let state = node.state_msg.get_root_as_reader::<fallback_state::Reader<'_>>().unwrap();
        current_index = state.get_current_index() as usize;
    }

    let child_count = node.children.len();

    if current_index >= child_count {
        return Tick::Failure(false);
    }

    let child_result = super::tick(&mut node.children[current_index], cmd);

    match child_result {
        Tick::Running(()) => {
            Tick::Running(())
        }
        Tick::Success(_) => {
            info!("[#{}] FALLBACK \"{}\" child {} succeeded -- done",
                node.id, node.name, current_index);
            Tick::Success(true)
        }
        Tick::Failure(_) => {
            let next = current_index + 1;
            info!("[#{}] FALLBACK \"{}\" child {} failed, trying next ({}/{})",
                node.id, node.name, current_index, next, child_count);

            {
                let mut state = node.state_msg.get_root::<fallback_state::Builder<'_>>().unwrap();
                state.set_current_index(next as u32);
            }

            if next >= child_count {
                info!("[#{}] FALLBACK \"{}\" all children failed", node.id, node.name);
                Tick::Failure(false)
            } else {
                Tick::Running(())
            }
        }
    }
}
