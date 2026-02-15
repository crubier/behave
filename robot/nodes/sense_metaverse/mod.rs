//! Sense Metaverse node -- subscribes to SimStatus and forwards as SenseStatus.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::topics;
use behave::topics::sense::status::SenseStatus;

pub fn run() -> Result<()> {
    behave::logging::init("SenseMeta");
    info!("starting (metaverse sim -> sense bridge)");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let sim_sub = topics::sim::status::subscribe(&node)?;
    info!("subscribed to {}", topics::sim::status::NAME);

    let sense_pub = topics::sense::status::publish(&node)?;
    info!("publishing {}", topics::sense::status::NAME);

    info!("ready");

    while node.wait(Duration::from_millis(10)).is_ok() {
        while let Some(status) = topics::receive_native(&sim_sub)? {
            topics::publish(&sense_pub, SenseStatus {
                easting_m: status.pose.x,
                northing_m: status.pose.y,
                altitude_m: status.pose.z,
            })?;
        }
    }

    warn!("node loop exited");
    Ok(())
}
