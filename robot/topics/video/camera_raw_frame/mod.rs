//! Topic: behave/video/CameraRawFrame
//!
//! Raw BGRA camera frames from the simulator or real camera.
//! Fixed resolution 640x480, 4 bytes per pixel (BGRA8).

use iceoryx2::prelude::*;

use crate::topics::{IoxNode, NativePub, NativeSub};

pub const NAME: &str = "behave/video/CameraRawFrame";

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;
pub const BYTES_PER_PIXEL: u32 = 4;
pub const FRAME_SIZE: usize = (WIDTH * HEIGHT * BYTES_PER_PIXEL) as usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, ZeroCopySend)]
pub struct CameraRawFrame {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub utime: u64,
    pub data: [u8; FRAME_SIZE],
}

impl Default for CameraRawFrame {
    fn default() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            stride: WIDTH * BYTES_PER_PIXEL,
            utime: 0,
            data: [0u8; FRAME_SIZE],
        }
    }
}

pub fn publish(node: &IoxNode) -> anyhow::Result<NativePub<CameraRawFrame>> {
    crate::topics::create_native_publisher::<CameraRawFrame>(node, NAME)
}

pub fn subscribe(node: &IoxNode) -> anyhow::Result<NativeSub<CameraRawFrame>> {
    crate::topics::create_native_subscriber::<CameraRawFrame>(node, NAME)
}
