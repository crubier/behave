//! RendererNode -- the "real DOM" node managed by the behavior tree renderer.
//!
//! Each Dioxus element becomes a RendererNode. The renderer maintains a
//! HashMap<ElementId, RendererNode> that mirrors the Dioxus element tree,
//! plus execution state managed by `NodeBehavior` trait objects.

use std::collections::HashMap;

use dioxus_core::ElementId;

use super::behavior::{ChildResult, NodeBehavior};

/// A node in the renderer's tree. One per Dioxus element.
pub struct RendererNode {
    /// Element tag name (e.g. "takeoff", "sequence").
    pub tag: &'static str,

    /// Dynamic attributes set on this element (name -> value).
    /// Only f64 values are tracked (action parameters).
    pub attrs: HashMap<&'static str, f64>,

    /// Parent element, if any. Root has None.
    pub parent: Option<ElementId>,

    /// Ordered child elements.
    pub children: Vec<ElementId>,

    /// Execution state.
    pub state: NodeState,
}

/// Execution state of a renderer node.
pub enum NodeState {
    /// Created by Dioxus but not yet activated by its parent.
    Idle,

    /// Currently executing. The `NodeBehavior` trait object drives
    /// the node's lifecycle (activate, tick, child-complete).
    Active(Box<dyn NodeBehavior>),

    /// Finished executing with a result.
    Completed(ChildResult),
}

impl RendererNode {
    /// Create a new idle node with the given tag.
    pub fn new(tag: &'static str) -> Self {
        Self {
            tag,
            attrs: HashMap::new(),
            parent: None,
            children: Vec::new(),
            state: NodeState::Idle,
        }
    }

    /// Whether this node is active (has a running NodeBehavior).
    pub fn is_active(&self) -> bool {
        matches!(self.state, NodeState::Active(_))
    }
}
