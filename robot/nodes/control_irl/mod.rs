//! Control IRL node -- real hardware flight controller interface.
//!
//! Stub: will eventually interface with real flight controller hardware.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::topics;

pub fn run() -> Result<()> {
    behave::logging::init("ControlIRL");
    info!("starting (stub -- no hardware connected)");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let _cmd_sub = topics::control::request::subscribe(&node)?;
    info!("subscribed to {}", topics::control::request::NAME);

    let _ack_pub = topics::control::response::publish(&node)?;
    let _state_pub = topics::control::status::publish(&node)?;

    info!("ready -- waiting for hardware integration");

    while node.wait(Duration::from_secs(5)).is_ok() {
        info!("heartbeat (no hardware)");
    }

    warn!("node loop exited");
    Ok(())
}
