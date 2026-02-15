//! Topic: behave/SimRequest
//!
//! Camera pose commands TO the simulator.
//! Schema: sim_request.capnp (colocated)

use crate::topics::{IoxNode, Pub, Sub};

pub const NAME: &str = "behave/SimRequest";
pub const BUF: usize = 256;

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
