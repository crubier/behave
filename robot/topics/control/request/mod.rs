//! Topic: behave/ControlRequest
//!
//! Flight commands from Behave -> Control.
//! Uses a flat struct with a command tag discriminant.

use iceoryx2::prelude::*;

use crate::topics::{IoxNode, NativePub, NativeSub};

pub const NAME: &str = "behave/ControlRequest";

// ── Command tags ────────────────────────────────────────────────

pub const CMD_ARM: u8 = 0;
pub const CMD_DISARM: u8 = 1;
pub const CMD_TAKEOFF: u8 = 2;
pub const CMD_LAND: u8 = 3;
pub const CMD_HOVER: u8 = 4;
pub const CMD_RETURN_HOME: u8 = 5;
pub const CMD_GOTO: u8 = 6;
pub const CMD_TRIGGER_CAMERA: u8 = 7;

const TAG_MAX: usize = 64;

// ── ControlRequest struct ───────────────────────────────────────

#[repr(C)]
#[derive(Debug, Clone, Copy, ZeroCopySend)]
pub struct ControlRequest {
    pub id: u64,
    pub cmd: u8,
    // Takeoff / ReturnHome
    pub altitude_m: f64,
    // Land
    pub descent_speed_ms: f64,
    // Goto
    pub easting_m: f64,
    pub northing_m: f64,
    pub speed_ms: f64,
    // TriggerCamera
    tag_len: u8,
    tag_buf: [u8; TAG_MAX],
}

impl Default for ControlRequest {
    fn default() -> Self {
        Self {
            id: 0, cmd: 0,
            altitude_m: 0.0, descent_speed_ms: 0.0,
            easting_m: 0.0, northing_m: 0.0, speed_ms: 0.0,
            tag_len: 0, tag_buf: [0u8; TAG_MAX],
        }
    }
}

impl ControlRequest {
    pub fn arm(id: u64) -> Self {
        Self { id, cmd: CMD_ARM, ..Default::default() }
    }

    pub fn disarm(id: u64) -> Self {
        Self { id, cmd: CMD_DISARM, ..Default::default() }
    }

    pub fn takeoff(id: u64, altitude_m: f64) -> Self {
        Self { id, cmd: CMD_TAKEOFF, altitude_m, ..Default::default() }
    }

    pub fn land(id: u64, descent_speed_ms: f64) -> Self {
        Self { id, cmd: CMD_LAND, descent_speed_ms, ..Default::default() }
    }

    pub fn hover(id: u64) -> Self {
        Self { id, cmd: CMD_HOVER, ..Default::default() }
    }

    pub fn return_home(id: u64, altitude_m: f64) -> Self {
        Self { id, cmd: CMD_RETURN_HOME, altitude_m, ..Default::default() }
    }

    pub fn goto(id: u64, easting_m: f64, northing_m: f64, altitude_m: f64, speed_ms: f64) -> Self {
        Self { id, cmd: CMD_GOTO, easting_m, northing_m, altitude_m, speed_ms, ..Default::default() }
    }

    pub fn trigger_camera(id: u64, tag: &str) -> Self {
        let bytes = tag.as_bytes();
        let len = bytes.len().min(TAG_MAX);
        let mut tag_buf = [0u8; TAG_MAX];
        tag_buf[..len].copy_from_slice(&bytes[..len]);
        Self { id, cmd: CMD_TRIGGER_CAMERA, tag_len: len as u8, tag_buf, ..Default::default() }
    }

    pub fn camera_tag(&self) -> &str {
        std::str::from_utf8(&self.tag_buf[..self.tag_len as usize]).unwrap_or("")
    }
}

pub fn publish(node: &IoxNode) -> anyhow::Result<NativePub<ControlRequest>> {
    crate::topics::create_native_publisher::<ControlRequest>(node, NAME)
}

pub fn subscribe(node: &IoxNode) -> anyhow::Result<NativeSub<ControlRequest>> {
    crate::topics::create_native_subscriber::<ControlRequest>(node, NAME)
}
