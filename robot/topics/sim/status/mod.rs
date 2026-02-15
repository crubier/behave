//! Topic: behave/SimStatus
//!
//! Telemetry FROM the simulator.

use iceoryx2::prelude::*;

use crate::topics::{IoxNode, NativePub, NativeSub};
use crate::topics::sim::CameraPose;

pub const NAME: &str = "behave/SimStatus";

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, ZeroCopySend)]
pub struct SimStatus {
    pub pose: CameraPose,
    pub utime: u64,
}

pub fn publish(node: &IoxNode) -> anyhow::Result<NativePub<SimStatus>> {
    crate::topics::create_native_publisher::<SimStatus>(node, NAME)
}

pub fn subscribe(node: &IoxNode) -> anyhow::Result<NativeSub<SimStatus>> {
    crate::topics::create_native_subscriber::<SimStatus>(node, NAME)
}
