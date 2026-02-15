//! Render a protobuf `ActionNode` tree as Dioxus elements.
//!
//! Bridge between the wire format (protobuf) and the Dioxus renderer.
//! Deserializes `ActionNode`, encodes each node's Args as `ProtoBytes`,
//! and renders the matching core element.

use dioxus::prelude::*;
use prost::Message;

use crate::actions_dioxus::action_proto::{action_args, ActionNode};
use crate::actions_dioxus::core::data::ProtoBytes;

#[allow(non_snake_case, unused)]
mod dioxus_elements {
    pub use crate::actions_dioxus::core::elements::*;
}

/// Recursively render a protobuf `ActionNode` tree as Dioxus elements.
#[component]
pub fn RenderActionNode(node: ActionNode) -> Element {
    let args = match &node.args {
        Some(a) => a,
        None => return rsx! {},
    };

    let action = match &args.action {
        Some(a) => a,
        None => return rsx! {},
    };

    match action {
        action_args::Action::Sequence(a) => rsx! {
            sequence { args: ProtoBytes(a.encode_to_vec()),
                for child in node.children.iter() { RenderActionNode { node: child.clone() } }
            }
        },
        action_args::Action::Fallback(a) => rsx! {
            fallback { args: ProtoBytes(a.encode_to_vec()),
                for child in node.children.iter() { RenderActionNode { node: child.clone() } }
            }
        },
        action_args::Action::Parallel(a) => rsx! {
            parallel { args: ProtoBytes(a.encode_to_vec()),
                for child in node.children.iter() { RenderActionNode { node: child.clone() } }
            }
        },
        action_args::Action::Takeoff(a) => rsx! {
            takeoff { args: ProtoBytes(a.encode_to_vec()) }
        },
        action_args::Action::Land(a) => rsx! {
            land { args: ProtoBytes(a.encode_to_vec()) }
        },
        action_args::Action::Goto(a) => rsx! {
            goto { args: ProtoBytes(a.encode_to_vec()) }
        },
        action_args::Action::Home(a) => rsx! {
            home { args: ProtoBytes(a.encode_to_vec()) }
        },
        action_args::Action::Photo(a) => rsx! {
            photo { args: ProtoBytes(a.encode_to_vec()) }
        },
    }
}

