//! Topic: behave/ActionRequest
//!
//! ActionArgs trees sent from Communicate -> Behave.
//! No wrapper -- raw ActionArgs capnp message on the wire.

use crate::topics::{IoxNode, Pub, Sub};

pub const NAME: &str = "behave/ActionRequest";
pub const BUF: usize = 65536;

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
