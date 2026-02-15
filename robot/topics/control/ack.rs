//! Topic: behave/ControlAck
//!
//! Acknowledgments sent from Control -> Behave after each command.

use crate::topics::{IoxNode, Pub, Sub};

pub const NAME: &str = "behave/ControlAck";
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
