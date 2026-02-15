//! Fallback composite -- ticks children left-to-right, succeeds if ANY succeeds.

use log::info;

use super::{ActionIO, ActionNode, Tick};

pub fn start(node: &mut ActionNode) {
    let child_count = node.children.len();
    info!("[#{}] FALLBACK \"{}\" start ({} children)", node.id, node.name, child_count);
    node.current_index = 0;
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), bool> {
    let idx = node.current_index;
    let child_count = node.children.len();

    if idx >= child_count {
        return Tick::Failure(false);
    }

    let child_result = super::tick(&mut node.children[idx], io);

    match child_result {
        Tick::Running(()) => Tick::Running(()),
        Tick::Success(_) => {
            info!("[#{}] FALLBACK \"{}\" child {} succeeded -- done",
                node.id, node.name, idx);
            Tick::Success(true)
        }
        Tick::Failure(_) => {
            let next = idx + 1;
            info!("[#{}] FALLBACK \"{}\" child {} failed, trying next ({}/{})",
                node.id, node.name, idx, next, child_count);
            node.current_index = next;

            if next >= child_count {
                info!("[#{}] FALLBACK \"{}\" all children failed", node.id, node.name);
                Tick::Failure(false)
            } else {
                Tick::Running(())
            }
        }
    }
}
