//! Sequence node -- succeeds if ALL children succeed (left-to-right).

use crate::actions_dioxus::ActionIO;
use crate::actions_dioxus::core::behavior::{ActionNode, Behavior, ChildResult, NodeResponse};

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.sequence.rs"));
}
use proto::*;

// ── Component ───────────────────────────────────────────────────

use dioxus::prelude::*;
#[allow(non_snake_case, unused)]
mod dioxus_elements {
    pub use crate::actions_dioxus::core::elements::*;
}

#[component]
pub fn Sequence(children: Element) -> Element {
    rsx! { sequence { {children} } }
}

// ── Node ────────────────────────────────────────────────────────

pub type SequenceNode = ActionNode<SequenceArgs, SequenceOutput, SequenceResult, SequenceInput, SequenceState>;

impl Behavior for SequenceNode {
    fn on_activate(&mut self, _io: &ActionIO) -> NodeResponse {
        NodeResponse::ActivateChild(0)
    }

    fn on_child_complete(&mut self, child_index: usize, child_count: usize, result: ChildResult) -> NodeResponse {
        self.output.current_index = child_index as u32;
        self.output.child_count = child_count as u32;
        self.state.current_index = self.output.current_index;
        self.state.child_count = self.output.child_count;

        match result {
            ChildResult::Success => {
                self.result.children_completed = (child_index + 1) as u32;
                let next = child_index + 1;
                if next < child_count {
                    self.output.current_index = next as u32;
                    self.state.current_index = next as u32;
                    NodeResponse::ActivateChild(next)
                } else {
                    NodeResponse::Success
                }
            }
            ChildResult::Failure => {
                self.result.children_completed = child_index as u32;
                self.result.failed_at_index = child_index as i32;
                NodeResponse::Failure
            }
        }
    }
}
