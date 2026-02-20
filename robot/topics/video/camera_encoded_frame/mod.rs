//! Topic: behave/video/CameraEncodedFrame
//!
//! H.264 encoded camera frames. Variable-size NAL unit data
//! inside a fixed-size IpcMessage envelope.

use crate::topics::{IoxNode, Pub, Sub};

pub const NAME: &str = "behave/video/CameraEncodedFrame";
pub const BUF: usize = 262144; // 256 KB max per encoded frame

pub fn publish(node: &IoxNode) -> anyhow::Result<Pub<BUF>> {
    crate::topics::create_publisher::<BUF>(node, NAME)
}

pub fn subscribe(node: &IoxNode) -> anyhow::Result<Sub<BUF>> {
    crate::topics::create_subscriber::<BUF>(node, NAME)
}
