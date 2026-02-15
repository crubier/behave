//! Sequence composite -- ticks children left-to-right, succeeds if ALL succeed.

use log::info;

use super::{ActionIO, ActionNode, ActionResultKind, Tick};
use crate::schema::behave::actions::SequenceResultArgs;

pub fn start(node: &mut ActionNode) {
    let child_count = node.children.len();
    info!("[#{}] SEQUENCE start ({} children)", node.id, child_count);
    node.current_index = 0;
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), ActionResultKind> {
    let idx = node.current_index;
    let child_count = node.children.len();

    if idx >= child_count {
        return Tick::Success(ActionResultKind::Sequence(SequenceResultArgs {
            success: true,
            children_completed: child_count as u32,
            failed_at_index: -1,
        }));
    }

    let child_result = super::tick(&mut node.children[idx], io);

    match child_result {
        Tick::Running(()) => {
            let progress = idx as f64 / child_count as f64 * 100.0;
            info!(
                "[#{}] SEQUENCE running child {}/{} ({:.0}%)",
                node.id, idx, child_count, progress
            );
            Tick::Running(())
        }
        Tick::Success(_) => {
            let next = idx + 1;
            info!(
                "[#{}] SEQUENCE child {} succeeded ({}/{})",
                node.id, idx, next, child_count
            );
            node.current_index = next;

            if next >= child_count {
                info!("[#{}] SEQUENCE all children done", node.id);
                Tick::Success(ActionResultKind::Sequence(SequenceResultArgs {
                    success: true,
                    children_completed: child_count as u32,
                    failed_at_index: -1,
                }))
            } else {
                Tick::Running(())
            }
        }
        Tick::Failure(_) => {
            info!("[#{}] SEQUENCE child {} FAILED -- aborting", node.id, idx);
            Tick::Failure(ActionResultKind::Sequence(SequenceResultArgs {
                success: false,
                children_completed: idx as u32,
                failed_at_index: idx as i32,
            }))
        }
    }
}
