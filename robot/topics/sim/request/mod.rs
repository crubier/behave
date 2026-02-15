//! Topic: behave/SimRequest
//!
//! Camera pose commands TO the simulator.

use iceoryx2::prelude::*;

use crate::topics::{IoxNode, NativePub, NativeSub};
use crate::topics::sim::CameraPose;

pub const NAME: &str = "behave/SimRequest";

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, ZeroCopySend)]
pub struct SimRequest {
    pub pose: CameraPose,
    pub utime: u64,
}

pub fn publish(node: &IoxNode) -> anyhow::Result<NativePub<SimRequest>> {
    crate::topics::create_native_publisher::<SimRequest>(node, NAME)
}

pub fn subscribe(node: &IoxNode) -> anyhow::Result<NativeSub<SimRequest>> {
    crate::topics::create_native_subscriber::<SimRequest>(node, NAME)
}
