//! Sense IRL node -- real hardware sensor data acquisition.
//!
//! Stub: will eventually interface with real sensor hardware
//! (e.g., IMU, GPS, barometer) and publish sensor state over iceoryx2.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

pub fn run() -> Result<()> {
    behave::logging::init("SenseIRL");
    info!("starting (stub -- no sensors connected)");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    info!("ready -- waiting for hardware integration");

    while node.wait(Duration::from_secs(5)).is_ok() {
        info!("heartbeat (no hardware)");
    }

    warn!("node loop exited");
    Ok(())
}
