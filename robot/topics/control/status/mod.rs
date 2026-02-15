//! Topic: behave/DroneState
//!
//! Telemetry from Control -> all nodes.
//! Schema: drone_state.capnp (colocated)

use crate::topics::{IoxNode, Pub, Sub};

pub const NAME: &str = "behave/DroneState";
pub const BUF: usize = 4096;

pub fn publish(node: &IoxNode) -> anyhow::Result<Pub<BUF>> {
    crate::topics::create_publisher::<BUF>(node, NAME)
}

pub fn subscribe(node: &IoxNode) -> anyhow::Result<Sub<BUF>> {
    crate::topics::create_subscriber::<BUF>(node, NAME)
}

pub fn send(
    pub_: &Pub<BUF>,
    builder: &capnp::message::Builder<capnp::message::HeapAllocator>,
) -> anyhow::Result<()> {
    crate::topics::send::<BUF>(pub_, builder)
}
