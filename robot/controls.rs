//! Controls API client -- helpers for sending ControlRequest messages.

use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::Result;

use crate::topics::control::request::ControlRequest;

static CMD_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Trait for sending control commands. Implemented by the iceoryx2 publisher
/// wrapper in the robot binary, but can be mocked for testing.
pub trait CmdPublisher {
    fn send_cmd(&self, cmd: ControlRequest) -> Result<()>;
}

fn next_cmd_id() -> u64 {
    CMD_COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn send_arm(pub_: &dyn CmdPublisher) -> Result<()> {
    pub_.send_cmd(ControlRequest::arm(next_cmd_id()))
}

pub fn send_takeoff(pub_: &dyn CmdPublisher, altitude_m: f64) -> Result<()> {
    pub_.send_cmd(ControlRequest::takeoff(next_cmd_id(), altitude_m))
}

pub fn send_goto(pub_: &dyn CmdPublisher, easting: f64, northing: f64, alt: f64, speed: f64) -> Result<()> {
    pub_.send_cmd(ControlRequest::goto(next_cmd_id(), easting, northing, alt, speed))
}

pub fn send_return_home(pub_: &dyn CmdPublisher, altitude_m: f64) -> Result<()> {
    pub_.send_cmd(ControlRequest::return_home(next_cmd_id(), altitude_m))
}

pub fn send_land(pub_: &dyn CmdPublisher, descent_speed_ms: f64) -> Result<()> {
    pub_.send_cmd(ControlRequest::land(next_cmd_id(), descent_speed_ms))
}

pub fn send_trigger_camera(pub_: &dyn CmdPublisher, tag: &str) -> Result<()> {
    pub_.send_cmd(ControlRequest::trigger_camera(next_cmd_id(), tag))
}
