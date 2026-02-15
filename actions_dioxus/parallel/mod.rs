//! Parallel node -- activates ALL children at once.
//!
//! - `on_activate`: activate all children simultaneously.
//! - `on_child_complete`: on failure -> fail immediately;
//!   on success -> succeed once all children have succeeded.

use std::collections::HashMap;

use crate::actions::io::ActionIO;
use crate::actions_dioxus::core::behavior::{ChildResult, NodeBehavior, NodeResponse};

pub struct ParallelNode {
    /// Tracks how many children have succeeded so far.
    succeeded_count: usize,
}

impl ParallelNode {
    pub fn from_attrs(_attrs: &HashMap<&'static str, f64>) -> Self {
        Self {
            succeeded_count: 0,
        }
    }
}

impl NodeBehavior for ParallelNode {
    fn on_activate(&mut self, _io: &ActionIO) -> NodeResponse {
        NodeResponse::ActivateAllChildren
    }

    fn on_child_complete(
        &mut self,
        _child_index: usize,
        child_count: usize,
        result: ChildResult,
    ) -> NodeResponse {
        match result {
            ChildResult::Failure => NodeResponse::Failure,
            ChildResult::Success => {
                self.succeeded_count += 1;
                if self.succeeded_count >= child_count {
                    NodeResponse::Success
                } else {
                    NodeResponse::Running
                }
            }
        }
    }
}
