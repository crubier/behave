//! Sim Mock node -- instant camera pose simulator.
//!
//! Subscribes to SimRequest and immediately publishes a SimStatus
//! with the requested pose as the current pose. No physics, no delay.

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};
use std::time::Duration;

use behave::schema::sim_request_capnp::sim_request;
use behave::topics;

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
        while let Some(typed) = topics::receive::<{ topics::sim::request::BUF }, sim_request::Owned>(&req_sub)? {
            let req = typed.get()?;
            let pose = req.get_pose()?;
            let utime = req.get_utime();

            info!(
                "pose -> ({:.2}, {:.2}, {:.2}) instantly",
                pose.get_x(), pose.get_y(), pose.get_z()
            );

            // Publish status with the exact requested pose
            let mut msg = capnp::message::Builder::new_default();
            {
                let mut status = msg.init_root::<behave::schema::sim_status_capnp::sim_status::Builder<'_>>();
                status.set_utime(utime);
                let mut out = status.init_pose();
                out.set_x(pose.get_x());
                out.set_y(pose.get_y());
                out.set_z(pose.get_z());
                out.set_qw(pose.get_qw());
                out.set_qx(pose.get_qx());
                out.set_qy(pose.get_qy());
                out.set_qz(pose.get_qz());
            }
            topics::sim::status::send(&status_pub, &msg)?;
        }
    }

    warn!("node loop exited");
    Ok(())
}
