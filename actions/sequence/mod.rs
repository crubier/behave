//! Sequence composite -- ticks children left-to-right, succeeds if ALL succeed.

use log::info;

use super::{ActionIO, ActionNode, Tick};

pub fn start(node: &mut ActionNode) {
    let child_count = node.children.len();
    info!("[#{}] SEQUENCE \"{}\" start ({} children)", node.id, node.name, child_count);
    node.current_index = 0;
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), bool> {
    let idx = node.current_index;
    let child_count = node.children.len();

    if idx >= child_count {
        return Tick::Success(true);
    }

    let child_result = super::tick(&mut node.children[idx], io);

    match child_result {
        Tick::Running(()) => {
            let progress = idx as f64 / child_count as f64 * 100.0;
            info!("[#{}] SEQUENCE \"{}\" running child {}/{} ({:.0}%)",
                node.id, node.name, idx, child_count, progress);
            Tick::Running(())
        }
        Tick::Success(_) => {
            let next = idx + 1;
            info!("[#{}] SEQUENCE \"{}\" child {} succeeded ({}/{})",
                node.id, node.name, idx, next, child_count);
            node.current_index = next;

            if next >= child_count {
                info!("[#{}] SEQUENCE \"{}\" all children done", node.id, node.name);
                Tick::Success(true)
            } else {
                Tick::Running(())
            }
        }
        Tick::Failure(_) => {
            info!("[#{}] SEQUENCE \"{}\" child {} FAILED -- aborting", node.id, node.name, idx);
            Tick::Failure(false)
        }
    }
}
