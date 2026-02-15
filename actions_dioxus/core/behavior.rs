//! Unified node behavior trait for behavior tree elements.
//!
//! Every core element (sequence, fallback, takeoff, land, etc.) implements
//! `NodeBehavior`. The composite/leaf distinction is emergent -- not enforced
//! by the type system:
//!
//! - A "leaf" overrides `on_activate` (send command) and `on_tick` (poll sensor).
//! - A "composite" overrides `on_activate` (activate first child) and
//!   `on_child_complete` (advance/propagate).
//! - A "hybrid" (e.g. while-decorator) can use all three.

use crate::actions::io::ActionIO;

/// What a node tells the renderer to do next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeResponse {
    /// Keep running -- tick me again next cycle.
    Running,
    /// Activate my child at this index.
    ActivateChild(usize),
    /// Activate all children simultaneously (parallel execution).
    ActivateAllChildren,
    /// I succeeded.
    Success,
    /// I failed.
    Failure,
}

/// Outcome of a child completing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildResult {
    Success,
    Failure,
}

/// The behavior of a core element in the behavior tree.
///
/// Implement this trait for each core element type (sequence, takeoff, etc.).
/// The renderer creates a `Box<dyn NodeBehavior>` when an element is activated,
/// and calls the appropriate method each cycle.
///
/// # Defaults
///
/// - `on_tick` returns `Running` (composites that only react to child results).
/// - `on_child_complete` propagates the child's result (leaves that have no children).
pub trait NodeBehavior {
    /// Called once when this node is activated by the renderer.
    ///
    /// - Leaves: send the initial command, return `Running`.
    /// - Composites: return `ActivateChild(0)` to start the first child.
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse;

    /// Called each tick while this node is active.
    ///
    /// Override for leaves that need to poll sensor state.
    /// Composites typically leave this as the default (`Running`).
    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        let _ = io;
        NodeResponse::Running
    }

    /// Called when a direct child completes with `result`.
    ///
    /// `child_index` is the index of the child that completed.
    /// `child_count` is the total number of children.
    ///
    /// Override for composites. The default propagates the child's
    /// result directly (useful for single-child wrappers/decorators).
    fn on_child_complete(
        &mut self,
        child_index: usize,
        child_count: usize,
        result: ChildResult,
    ) -> NodeResponse {
        let _ = (child_index, child_count);
        match result {
            ChildResult::Success => NodeResponse::Success,
            ChildResult::Failure => NodeResponse::Failure,
        }
    }
}
