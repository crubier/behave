//! Sim Mock node -- instant camera pose simulator.
//!
//! Subscribes to SimRequest and immediately publishes a SimStatus
//! with the requested pose as the current pose.

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};
use std::time::Duration;

use behave::topics;
use behave::topics::sim::status::SimStatus;

pub fn run() -> Result<()> {
    behave::logging::init("SimMock");
    info!("starting (instant pose -- no physics)");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let req_sub = topics::sim::request::subscribe(&node)?;
    info!("subscribed to {}", topics::sim::request::NAME);

    let status_pub = topics::sim::status::publish(&node)?;
    info!("publishing {}", topics::sim::status::NAME);

    info!("ready");

    while node.wait(Duration::from_millis(10)).is_ok() {
        while let Some(req) = topics::receive_native(&req_sub)? {
            info!("pose -> ({:.2}, {:.2}, {:.2}) instantly", req.pose.x, req.pose.y, req.pose.z);
            topics::publish(&status_pub, SimStatus { pose: req.pose, utime: req.utime })?;
        }
    }

    warn!("node loop exited");
    Ok(())
}
