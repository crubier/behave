//! Topic: behave/ActionRun
//!
//! Protobuf ActionRun tree published by Behave after each tick.
//! Contains the full recursive tree state for visualization
//! (e.g. Foxglove Raw Messages panel).

use crate::topics::{IoxNode, Pub, Sub};

pub const NAME: &str = "behave/ActionRun";
pub const BUF: usize = 131072; // 128 KB

pub fn publish(node: &IoxNode) -> anyhow::Result<Pub<BUF>> {
    crate::topics::create_publisher::<BUF>(node, NAME)
}

pub fn subscribe(node: &IoxNode) -> anyhow::Result<Sub<BUF>> {
    crate::topics::create_subscriber::<BUF>(node, NAME)
}
