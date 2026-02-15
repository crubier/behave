//! Parallel node -- activates ALL children at once.

use crate::actions::io::ActionIO;
use crate::actions_dioxus::core::behavior::{ActionNode, Behavior, ChildResult, NodeResponse};

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.parallel.rs"));
}
use proto::*;

// ── Component ───────────────────────────────────────────────────

use dioxus::prelude::*;
#[allow(non_snake_case, unused)]
mod dioxus_elements {
    pub use crate::actions_dioxus::core::elements::*;
}

#[component]
pub fn Parallel(children: Element) -> Element {
    rsx! { parallel { {children} } }
}

// ── Node ────────────────────────────────────────────────────────

pub type ParallelNode = ActionNode<ParallelArgs, ParallelOutput, ParallelResult, ParallelState>;

impl Behavior for ParallelNode {
    fn on_activate(&mut self, _io: &ActionIO) -> NodeResponse {
        NodeResponse::ActivateAllChildren
    }

    fn on_child_complete(&mut self, _child_index: usize, child_count: usize, result: ChildResult) -> NodeResponse {
        self.output.child_count = child_count as u32;
        self.state.child_count = self.output.child_count;

        match result {
            ChildResult::Failure => NodeResponse::Failure,
            ChildResult::Success => {
                self.output.succeeded_count += 1;
                self.result.succeeded_count = self.output.succeeded_count;
                self.result.child_count = self.output.child_count;
                self.state.succeeded_count = self.output.succeeded_count;

                if self.output.succeeded_count >= self.output.child_count {
                    NodeResponse::Success
                } else {
                    NodeResponse::Running
                }
            }
        }
    }
}
