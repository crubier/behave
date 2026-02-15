//! Topic: behave/SenseStatus
//!
//! Fused navigation state from Sense -> all nodes.

use iceoryx2::prelude::*;

use crate::topics::{IoxNode, NativePub, NativeSub};

pub const NAME: &str = "behave/SenseStatus";

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, ZeroCopySend)]
pub struct SenseStatus {
    pub easting_m: f64,
    pub northing_m: f64,
    pub altitude_m: f64,
}

pub fn publish(node: &IoxNode) -> anyhow::Result<NativePub<SenseStatus>> {
    crate::topics::create_native_publisher::<SenseStatus>(node, NAME)
}

pub fn subscribe(node: &IoxNode) -> anyhow::Result<NativeSub<SenseStatus>> {
    crate::topics::create_native_subscriber::<SenseStatus>(node, NAME)
}
