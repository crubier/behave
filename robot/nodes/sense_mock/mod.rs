//! Sense Mock node -- subscribes to SimStatus and forwards as SenseStatus.
//!
//! Reads the simulated camera pose from SimStatus and publishes it
//! as vehicle navigation state on SenseStatus.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::schema::sim_status_capnp::sim_status;
use behave::topics;

pub fn run() -> Result<()> {
    behave::logging::init("SenseMock");
    info!("starting (sim -> sense bridge)");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let sim_sub = topics::sim::status::subscribe(&node)?;
    info!("subscribed to {}", topics::sim::status::NAME);

    let sense_pub = topics::sense::status::publish(&node)?;
    info!("publishing {}", topics::sense::status::NAME);

    info!("ready");

    while node.wait(Duration::from_millis(10)).is_ok() {
        while let Some(typed) = topics::receive::<{ topics::sim::status::BUF }, sim_status::Owned>(&sim_sub)? {
            let status = typed.get()?;
            let pose = status.get_pose()?;

            let mut msg = capnp::message::Builder::new_default();
            {
                let mut sense = msg.init_root::<behave::schema::sense_status_capnp::sense_status::Builder<'_>>();
                sense.set_easting_m(pose.get_x());
                sense.set_northing_m(pose.get_y());
                sense.set_altitude_m(pose.get_z());
            }
            topics::sense::status::send(&sense_pub, &msg)?;
        }
    }

    warn!("node loop exited");
    Ok(())
}
