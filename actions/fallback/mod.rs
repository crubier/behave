//! Fallback composite -- ticks children left-to-right, succeeds if ANY succeeds.

use log::info;

use super::{ActionIO, ActionNode, ActionResultKind, Tick};
use crate::schema::behave::actions::FallbackResultArgs;

pub fn start(node: &mut ActionNode) {
    let child_count = node.children.len();
    info!("[#{}] FALLBACK start ({} children)", node.id, child_count);
    node.current_index = 0;
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), ActionResultKind> {
    let idx = node.current_index;
    let child_count = node.children.len();

    if idx >= child_count {
        return Tick::Failure(ActionResultKind::Fallback(FallbackResultArgs {
            success: false,
            succeeded_at_index: -1,
            children_attempted: child_count as u32,
        }));
    }

    let child_result = super::tick(&mut node.children[idx], io);

    match child_result {
        Tick::Running(()) => Tick::Running(()),
        Tick::Success(_) => {
            info!("[#{}] FALLBACK child {} succeeded -- done", node.id, idx);
            Tick::Success(ActionResultKind::Fallback(FallbackResultArgs {
                success: true,
                succeeded_at_index: idx as i32,
                children_attempted: (idx + 1) as u32,
            }))
        }
        Tick::Failure(_) => {
            let next = idx + 1;
            info!(
                "[#{}] FALLBACK child {} failed, trying next ({}/{})",
                node.id, idx, next, child_count
            );
            node.current_index = next;

            if next >= child_count {
                info!("[#{}] FALLBACK all children failed", node.id);
                Tick::Failure(ActionResultKind::Fallback(FallbackResultArgs {
                    success: false,
                    succeeded_at_index: -1,
                    children_attempted: child_count as u32,
                }))
            } else {
                Tick::Running(())
            }
        }
    }
}
