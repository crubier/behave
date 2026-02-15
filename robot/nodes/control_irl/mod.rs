//! Control IRL node -- real hardware flight controller interface.
//!
//! Stub: will eventually interface with real flight controller hardware
//! (e.g., MAVLink, UART, CAN bus).
//! Subscribes to the same topics as control_sim.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::topics;

pub fn run() -> Result<()> {
    behave::logging::init("ControlIRL");
    info!("starting (stub -- no hardware connected)");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let _cmd_sub = topics::control::command::subscribe(&node)?;
    info!("subscribed to {}", topics::control::command::NAME);

    let _ack_pub = topics::control::ack::publish(&node)?;
    let _state_pub = topics::control::state::publish(&node)?;

    info!("ready -- waiting for hardware integration");

    while node.wait(Duration::from_secs(5)).is_ok() {
        info!("heartbeat (no hardware)");
    }

    warn!("node loop exited");
    Ok(())
}
