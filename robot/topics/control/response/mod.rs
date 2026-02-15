//! Topic: behave/ControlResponse
//!
//! Command acknowledgments from Control -> Behave.

use iceoryx2::prelude::*;

use crate::topics::{IoxNode, NativePub, NativeSub};

pub const NAME: &str = "behave/ControlResponse";

const MSG_MAX: usize = 256;

#[repr(C)]
#[derive(Debug, Clone, Copy, ZeroCopySend)]
pub struct ControlResponse {
    pub command_id: u64,
    pub success: bool,
    msg_len: u8,
    msg_buf: [u8; MSG_MAX],
}

impl Default for ControlResponse {
    fn default() -> Self {
        Self {
            command_id: 0,
            success: false,
            msg_len: 0,
            msg_buf: [0u8; MSG_MAX],
        }
    }
}

impl ControlResponse {
    pub fn new(command_id: u64, success: bool, message: &str) -> Self {
        let bytes = message.as_bytes();
        let len = bytes.len().min(MSG_MAX);
        let mut msg_buf = [0u8; MSG_MAX];
        msg_buf[..len].copy_from_slice(&bytes[..len]);
        Self { command_id, success, msg_len: len as u8, msg_buf }
    }

    pub fn message(&self) -> &str {
        std::str::from_utf8(&self.msg_buf[..self.msg_len as usize]).unwrap_or("")
    }
}

pub fn publish(node: &IoxNode) -> anyhow::Result<NativePub<ControlResponse>> {
    crate::topics::create_native_publisher::<ControlResponse>(node, NAME)
}

pub fn subscribe(node: &IoxNode) -> anyhow::Result<NativeSub<ControlResponse>> {
    crate::topics::create_native_subscriber::<ControlResponse>(node, NAME)
}
