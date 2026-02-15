//! Sense Sim node -- simulated sensor data acquisition.
//!
//! Stub: will eventually publish simulated sensor state (IMU, GPS, battery,
//! etc.) over iceoryx2 for other nodes to consume.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::info;

pub fn run() -> Result<()> {
    behave::logging::init("SenseSim");
    info!("starting (simulated sensors)");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    info!("ready");

    loop {
        if node.wait(Duration::from_secs(5)).is_err() {
            break;
        }
        info!("heartbeat");
    }

    Ok(())
}
