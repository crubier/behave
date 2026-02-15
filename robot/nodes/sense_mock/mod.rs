//! Sense Mock node -- dumb stub that just emits periodic heartbeats.
//!
//! No sensor data, no simulation. Suitable for lightweight testing.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::info;

pub fn run() -> Result<()> {
    behave::logging::init("SenseMock");
    info!("starting (dumb mock -- heartbeats only)");

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
