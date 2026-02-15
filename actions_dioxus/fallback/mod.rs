//! Fallback node -- succeeds if ANY child succeeds (left-to-right).
//!
//! - `on_activate`: activate first child.
//! - `on_child_complete`: on success -> succeed; on failure -> next child.

use std::collections::HashMap;

use crate::actions::io::ActionIO;
use crate::actions_dioxus::core::behavior::{ChildResult, NodeBehavior, NodeResponse};

pub struct FallbackNode;

impl FallbackNode {
    pub fn from_attrs(_attrs: &HashMap<&'static str, f64>) -> Self {
        Self
    }
}

impl NodeBehavior for FallbackNode {
    fn on_activate(&mut self, _io: &ActionIO) -> NodeResponse {
        NodeResponse::ActivateChild(0)
    }

    fn on_child_complete(
        &mut self,
        child_index: usize,
        child_count: usize,
        result: ChildResult,
    ) -> NodeResponse {
        match result {
            ChildResult::Success => NodeResponse::Success,
            ChildResult::Failure => {
                let next = child_index + 1;
                if next < child_count {
                    NodeResponse::ActivateChild(next)
                } else {
                    NodeResponse::Failure
                }
            }
        }
    }
}
