//! RendererNode -- the "real DOM" node managed by the behavior tree renderer.

use dioxus_core::ElementId;

use super::behavior::{ChildResult, NodeBehavior};
use super::data::{Run, RunId};

/// A node in the renderer's tree. One per Dioxus element.
pub struct RendererNode {
    /// Element tag name (e.g. "takeoff", "sequence").
    pub tag: &'static str,

    /// Serialized protobuf Args, set via the `args` attribute.
    pub args_bytes: Vec<u8>,

    /// Parent element, if any. Root has None.
    pub parent: Option<ElementId>,

    /// Ordered child elements.
    pub children: Vec<ElementId>,

    /// Execution state.
    pub state: NodeState,

    /// The current Run record for this node, if activated.
    pub run: Option<Run>,
}

/// Execution state of a renderer node.
pub enum NodeState {
    /// Created by Dioxus but not yet activated.
    Idle,

    /// Currently executing.
    Active(Box<dyn NodeBehavior>),

    /// Finished executing.
    Completed(ChildResult),
}

impl RendererNode {
    /// Create a new idle node with the given tag.
    pub fn new(tag: &'static str) -> Self {
        Self {
            tag,
            args_bytes: Vec::new(),
            parent: None,
            children: Vec::new(),
            state: NodeState::Idle,
            run: None,
        }
    }

    /// Whether this node is active.
    pub fn is_active(&self) -> bool {
        matches!(self.state, NodeState::Active(_))
    }

    /// Start a new Run record.
    pub fn start_run(&mut self, id: RunId, started_at: super::data::Utime) {
        self.run = Some(Run::new(id, self.tag, started_at));
    }
}
