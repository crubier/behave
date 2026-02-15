//! Test utility: publishes camera poses on the `behave/SimRequest` service.
//!
//! Usage:  cargo run --bin send_pose
//!
//! Sends a slow circular orbit so you can verify the UE5 camera is moving.

use std::time::{Instant, SystemTime, UNIX_EPOCH};

use iceoryx2::prelude::*;
use sim_bridge::{pack, IpcPoseMessage};

mod sim_capnp {
    include!(concat!(env!("OUT_DIR"), "/sim_capnp.rs"));
}

fn main() -> anyhow::Result<()> {
    let node = NodeBuilder::new()
        .name(&"send_pose".try_into()?)
        .create::<iceoryx2::prelude::ipc::Service>()?;

    let service = node
        .service_builder(&"behave/SimRequest".try_into()?)
        .publish_subscribe::<IpcPoseMessage>()
        .open_or_create()?;

    let publisher = service.publisher_builder().create()?;

    eprintln!("[send_pose] publishing on behave/SimRequest  (Ctrl-C to stop)");

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
            let mut pose = builder.init_root::<sim_capnp::camera_pose::Builder>();
            pose.set_x(x);
            pose.set_y(y);
            pose.set_z(z);
            pose.set_qw(qw);
            pose.set_qx(0.0);
            pose.set_qy(0.0);
            pose.set_qz(qz);
            pose.set_timestamp_us(now_us);
        }

        let msg = pack(&builder)?;
        let sample = publisher.loan_uninit()?;
        let sample = sample.write_payload(msg);
        sample.send()?;

        eprintln!(
            "[send_pose] t={t:.1}s  pos=({x:.2}, {y:.2}, {z:.2})  yaw={yaw:.2}rad"
        );

        std::thread::sleep(period);
    }
}
