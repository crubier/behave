//! Test utility: publishes camera poses on the `behave/SimRequest` service.
//!
//! Usage:  cargo run --bin send_pose
//!
//! Sends a slow circular orbit so you can verify the UE5 camera is moving.
//! The sim_metaverse node will interpolate and publish SimStatus,
//! which the sim_bridge (inside UE5) subscribes to.

use std::time::{Instant, SystemTime, UNIX_EPOCH};

use iceoryx2::prelude::*;

use behave::schema::sim_request_capnp;
use behave::topics;

fn main() -> anyhow::Result<()> {
    let node = NodeBuilder::new()
        .name(&"send_pose".try_into()?)
        .create::<iceoryx2::prelude::ipc::Service>()?;

    let pub_ = topics::sim::request::publish(&node)?;

    eprintln!("[send_pose] publishing on {}  (Ctrl-C to stop)", topics::sim::request::NAME);

    let start = Instant::now();
    let period = std::time::Duration::from_millis(50); // 20 Hz

    loop {
        let t = start.elapsed().as_secs_f64();

        // Circular orbit: radius 5 m, height 2 m, one revolution per 10 s
        let angle = t * std::f64::consts::TAU / 10.0;
        let x = 5.0 * angle.cos();
        let y = 5.0 * angle.sin();
        let z = 2.0;

        // Face the center: yaw = angle + pi
        let yaw = angle + std::f64::consts::PI;
        let half = yaw / 2.0;
        let qw = half.cos();
        let qz = half.sin();

        let now_us = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        // Build capnp message
        let mut builder = capnp::message::Builder::new_default();
        {
            let mut req = builder.init_root::<sim_request_capnp::sim_request::Builder>();
            req.set_utime(now_us);
            let mut pose = req.init_pose();
            pose.set_x(x);
            pose.set_y(y);
            pose.set_z(z);
            pose.set_qw(qw);
            pose.set_qx(0.0);
            pose.set_qy(0.0);
            pose.set_qz(qz);
        }

        topics::sim::request::send(&pub_, &builder)?;

        eprintln!(
            "[send_pose] t={t:.1}s  pos=({x:.2}, {y:.2}, {z:.2})  yaw={yaw:.2}rad"
        );

        std::thread::sleep(period);
    }
}
