//! Sim Metaverse node -- UE5 metaverse camera simulator.
//!
//! Subscribes to SimRequest, simulates simple physics by linearly
//! interpolating position and quaternion, publishes SimStatus at 60 Hz,
//! and sends a UDP pose packet to UE5.

use std::net::UdpSocket;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::topics;
use behave::topics::sim::CameraPose;
use behave::topics::sim::status::SimStatus;

/// UDP port for sending pose to UE5.
const UE5_UDP_PORT: u16 = 9876;

/// Flat pose packet sent over UDP to UE5 (64 bytes, little-endian).
#[repr(C, packed)]
struct UdpPosePacket {
    x: f64, y: f64, z: f64,
    qw: f64, qx: f64, qy: f64, qz: f64,
    utime: u64,
}

const TICK_HZ: u64 = 60;
const TICK_DT: f64 = 1.0 / TICK_HZ as f64;
const DEFAULT_LINEAR_SPEED: f64 = 2.0;
const DEFAULT_ANGULAR_SPEED: f64 = 1.0;

fn env_f64(key: &str, default: f64) -> f64 {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

struct Pose { x: f64, y: f64, z: f64, qw: f64, qx: f64, qy: f64, qz: f64 }

impl Pose {
    fn origin() -> Self { Self { x: 0.0, y: 0.0, z: 0.0, qw: 1.0, qx: 0.0, qy: 0.0, qz: 0.0 } }
}

fn move_toward(current: f64, target: f64, max_step: f64) -> f64 {
    let diff = target - current;
    if diff.abs() <= max_step { target } else { current + diff.signum() * max_step }
}

fn qdot(a: &Pose, b: &Pose) -> f64 { a.qw*b.qw + a.qx*b.qx + a.qy*b.qy + a.qz*b.qz }

fn qnorm(p: &mut Pose) {
    let len = (p.qw*p.qw + p.qx*p.qx + p.qy*p.qy + p.qz*p.qz).sqrt();
    if len > 1e-12 { p.qw /= len; p.qx /= len; p.qy /= len; p.qz /= len; }
}

fn slerp_step(cur: &mut Pose, tgt: &Pose, max_angle: f64) {
    let dot = qdot(cur, tgt).clamp(-1.0, 1.0);
    let angle = dot.abs().acos() * 2.0;
    if angle < 1e-6 { cur.qw = tgt.qw; cur.qx = tgt.qx; cur.qy = tgt.qy; cur.qz = tgt.qz; return; }
    let t = (max_angle / angle).min(1.0);
    let sign = if dot < 0.0 { -1.0 } else { 1.0 };
    cur.qw = cur.qw*(1.0-t) + tgt.qw*sign*t;
    cur.qx = cur.qx*(1.0-t) + tgt.qx*sign*t;
    cur.qy = cur.qy*(1.0-t) + tgt.qy*sign*t;
    cur.qz = cur.qz*(1.0-t) + tgt.qz*sign*t;
    qnorm(cur);
}

fn now_us() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64 }

pub fn run() -> Result<()> {
    behave::logging::init("SimMeta");

    let linear_speed = env_f64("BEHAVE_MAX_LINEAR_SPEED", DEFAULT_LINEAR_SPEED);
    let angular_speed = env_f64("BEHAVE_MAX_ANGULAR_SPEED", DEFAULT_ANGULAR_SPEED);

    info!("starting (metaverse sim -- 60 Hz, lin={linear_speed:.1}m/s, ang={angular_speed:.2}rad/s)");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let req_sub = topics::sim::request::subscribe(&node)?;
    info!("subscribed to {}", topics::sim::request::NAME);

    let status_pub = topics::sim::status::publish(&node)?;
    info!("publishing {}", topics::sim::status::NAME);

    let udp = UdpSocket::bind("0.0.0.0:0")?;
    let ue5_addr = format!("127.0.0.1:{UE5_UDP_PORT}");
    info!("sending UDP pose to {ue5_addr}");

    let mut current = Pose::origin();
    let mut target = Pose::origin();

    info!("ready");

    let tick = Duration::from_secs(1) / TICK_HZ as u32;

    loop {
        let t0 = Instant::now();

        // Drain incoming requests -- keep the latest
        while let Some(req) = topics::receive_native(&req_sub)? {
            target = Pose {
                x: req.pose.x, y: req.pose.y, z: req.pose.z,
                qw: req.pose.qw, qx: req.pose.qx, qy: req.pose.qy, qz: req.pose.qz,
            };
        }

        // Step position
        let lin_step = linear_speed * TICK_DT;
        current.x = move_toward(current.x, target.x, lin_step);
        current.y = move_toward(current.y, target.y, lin_step);
        current.z = move_toward(current.z, target.z, lin_step);

        // Step orientation
        let ang_step = angular_speed * TICK_DT;
        slerp_step(&mut current, &target, ang_step);

        let utime = now_us();

        // Publish status via iceoryx2
        topics::publish(&status_pub, SimStatus {
            pose: CameraPose {
                x: current.x, y: current.y, z: current.z,
                qw: current.qw, qx: current.qx, qy: current.qy, qz: current.qz,
            },
            utime,
        })?;

        // Send pose to UE5 over UDP
        {
            let pkt = UdpPosePacket {
                x: current.x, y: current.y, z: current.z,
                qw: current.qw, qx: current.qx, qy: current.qy, qz: current.qz,
                utime,
            };
            let bytes: &[u8] = unsafe {
                std::slice::from_raw_parts(&pkt as *const UdpPosePacket as *const u8, std::mem::size_of::<UdpPosePacket>())
            };
            let _ = udp.send_to(bytes, &ue5_addr);
        }

        let elapsed = t0.elapsed();
        if elapsed < tick { std::thread::sleep(tick - elapsed); }
    }
}
