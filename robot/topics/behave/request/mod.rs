//! Topic: behave/ActionRequest
//!
//! FlatBuffer ActionNode trees sent from Communicate -> Behave
//! inside IpcMessage envelopes.

use crate::topics::{IoxNode, Pub, Sub};

pub const NAME: &str = "behave/ActionRequest";
pub const BUF: usize = 65536;

pub fn publish(node: &IoxNode) -> anyhow::Result<Pub<BUF>> {
    crate::topics::create_publisher::<BUF>(node, NAME)
}

pub fn subscribe(node: &IoxNode) -> anyhow::Result<Sub<BUF>> {
    crate::topics::create_subscriber::<BUF>(node, NAME)
}
