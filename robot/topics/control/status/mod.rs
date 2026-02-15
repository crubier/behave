//! Topic: behave/ControlStatus
//!
//! Flight controller state from Control -> all nodes.

use iceoryx2::prelude::*;

use crate::topics::{IoxNode, NativePub, NativeSub};

pub const NAME: &str = "behave/ControlStatus";

pub const MODE_IDLE: u8 = 0;
pub const MODE_TAKING_OFF: u8 = 1;
pub const MODE_FLYING: u8 = 2;
pub const MODE_LANDING: u8 = 3;
pub const MODE_HOVERING: u8 = 4;
pub const MODE_RETURNING: u8 = 5;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, ZeroCopySend)]
pub struct ControlStatus {
    pub armed: bool,
    pub mode: u8,
    pub battery_pct: f64,
}

pub fn publish(node: &IoxNode) -> anyhow::Result<NativePub<ControlStatus>> {
    crate::topics::create_native_publisher::<ControlStatus>(node, NAME)
}

pub fn subscribe(node: &IoxNode) -> anyhow::Result<NativeSub<ControlStatus>> {
    crate::topics::create_native_subscriber::<ControlStatus>(node, NAME)
}
