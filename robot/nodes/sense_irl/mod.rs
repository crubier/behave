//! Sense IRL node -- real hardware sensor data acquisition.
//!
//! Stub: will eventually read from real sensors (IMU, GPS, barometer)
//! and publish fused navigation state on SenseStatus.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::topics;

pub fn run() -> Result<()> {
    behave::logging::init("SenseIRL");
    info!("starting (stub -- no sensors connected)");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let _sense_pub = topics::sense::status::publish(&node)?;
    info!("publishing {}", topics::sense::status::NAME);

    info!("ready -- waiting for hardware integration");

    while node.wait(Duration::from_secs(5)).is_ok() {
        info!("heartbeat (no hardware)");
    }

    warn!("node loop exited");
    Ok(())
}
