//! Controls API client -- helpers for sending ControlCommand messages.

use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::Result;

use crate::ipc::CmdMessage;
use crate::schema::control_command_capnp::control_command;
use crate::topics;

static CMD_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Trait for sending control commands. Implemented by the iceoryx2 publisher
/// wrapper in the robot binary, but can be mocked for testing.
pub trait CmdPublisher {
    fn send_envelope(&self, envelope: CmdMessage) -> Result<()>;
}

fn next_cmd_id() -> u64 {
    CMD_COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn send_arm(pub_: &dyn CmdPublisher) -> Result<()> {
    let mut msg = capnp::message::Builder::new_default();
    { let mut cmd = msg.init_root::<control_command::Builder<'_>>(); cmd.set_id(next_cmd_id()); cmd.set_arm(()); }
    pub_.send_envelope(crate::ipc::pack::<{ topics::control::request::BUF }>(&msg)?)
}

pub fn send_takeoff(pub_: &dyn CmdPublisher, altitude_m: f64) -> Result<()> {
    let mut msg = capnp::message::Builder::new_default();
    { let mut cmd = msg.init_root::<control_command::Builder<'_>>(); cmd.set_id(next_cmd_id()); cmd.init_takeoff().set_altitude_m(altitude_m); }
    pub_.send_envelope(crate::ipc::pack::<{ topics::control::request::BUF }>(&msg)?)
}

pub fn send_goto(pub_: &dyn CmdPublisher, lat: f64, lon: f64, alt: f64, speed: f64) -> Result<()> {
    let mut msg = capnp::message::Builder::new_default();
    {
        let mut cmd = msg.init_root::<control_command::Builder<'_>>();
        cmd.set_id(next_cmd_id());
        let mut g = cmd.init_goto();
        g.set_latitude_deg(lat);
        g.set_longitude_deg(lon);
        g.set_altitude_m(alt);
        g.set_speed_ms(speed);
    }
    pub_.send_envelope(crate::ipc::pack::<{ topics::control::request::BUF }>(&msg)?)
}

pub fn send_return_home(pub_: &dyn CmdPublisher, altitude_m: f64) -> Result<()> {
    let mut msg = capnp::message::Builder::new_default();
    { let mut cmd = msg.init_root::<control_command::Builder<'_>>(); cmd.set_id(next_cmd_id()); cmd.init_return_home().set_altitude_m(altitude_m); }
    pub_.send_envelope(crate::ipc::pack::<{ topics::control::request::BUF }>(&msg)?)
}

pub fn send_land(pub_: &dyn CmdPublisher, descent_speed_ms: f64) -> Result<()> {
    let mut msg = capnp::message::Builder::new_default();
    { let mut cmd = msg.init_root::<control_command::Builder<'_>>(); cmd.set_id(next_cmd_id()); cmd.init_land().set_descent_speed_ms(descent_speed_ms); }
    pub_.send_envelope(crate::ipc::pack::<{ topics::control::request::BUF }>(&msg)?)
}

pub fn send_trigger_camera(pub_: &dyn CmdPublisher, tag: &str) -> Result<()> {
    let mut msg = capnp::message::Builder::new_default();
    { let mut cmd = msg.init_root::<control_command::Builder<'_>>(); cmd.set_id(next_cmd_id()); cmd.init_trigger_camera().set_tag(tag.into()); }
    pub_.send_envelope(crate::ipc::pack::<{ topics::control::request::BUF }>(&msg)?)
}
