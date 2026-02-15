//! C FFI bridge: iceoryx2 camera-pose subscriber for UE5.
//!
//! A background thread subscribes to the `"behave/SimRequest"` iceoryx2 service
//! and stores the latest [`CameraPose`] Cap'n Proto message. The UE5 game
//! thread polls via the exported C functions.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;

use iceoryx2::prelude::*;

mod sim_capnp {
    include!(concat!(env!("OUT_DIR"), "/sim_capnp.rs"));
}

// ── IPC message envelope (mirrors robot/ipc.rs) ──────────────────

const POSE_BUF: usize = 256;

#[repr(C)]
#[derive(Clone, Debug, ZeroCopySend)]
pub struct IpcPoseMessage {
    pub len: u32,
    pub data: [u8; POSE_BUF],
}

impl Default for IpcPoseMessage {
    fn default() -> Self {
        Self {
            len: 0,
            data: [0u8; POSE_BUF],
        }
    }
}

/// Serialize a Cap'n Proto builder into an [`IpcPoseMessage`].
pub fn pack(
    builder: &capnp::message::Builder<capnp::message::HeapAllocator>,
) -> anyhow::Result<IpcPoseMessage> {
    let mut buf = Vec::new();
    capnp::serialize::write_message(&mut buf, builder)?;
    anyhow::ensure!(
        buf.len() <= POSE_BUF,
        "capnp message too large: {} bytes (max {POSE_BUF})",
        buf.len()
    );
    let mut msg = IpcPoseMessage::default();
    msg.len = buf.len() as u32;
    msg.data[..buf.len()].copy_from_slice(&buf);
    Ok(msg)
}

// ── C-compatible pose struct ─────────────────────────────────────

/// Camera pose exchanged across the FFI boundary (meters + quaternion).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CameraPoseC {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub qw: f64,
    pub qx: f64,
    pub qy: f64,
    pub qz: f64,
    pub timestamp_us: u64,
}

impl Default for CameraPoseC {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            qw: 1.0,
            qx: 0.0,
            qy: 0.0,
            qz: 0.0,
            timestamp_us: 0,
        }
    }
}

// ── Global state ─────────────────────────────────────────────────

struct PoseState {
    pose: CameraPoseC,
    has_new: bool,
}

static LATEST: Mutex<PoseState> = Mutex::new(PoseState {
    pose: CameraPoseC {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        qw: 1.0,
        qx: 0.0,
        qy: 0.0,
        qz: 0.0,
        timestamp_us: 0,
    },
    has_new: false,
});

static RUNNING: AtomicBool = AtomicBool::new(false);

// ── Background subscriber loop ──────────────────────────────────

fn bridge_loop() -> anyhow::Result<()> {
    let node = NodeBuilder::new()
        .name(&"sim_bridge".try_into()?)
        .create::<iceoryx2::prelude::ipc::Service>()?;

    let service = node
        .service_builder(&"behave/SimRequest".try_into()?)
        .publish_subscribe::<IpcPoseMessage>()
        .open_or_create()?;

    let subscriber = service.subscriber_builder().create()?;

    eprintln!("[sim_bridge] subscriber ready on behave/SimRequest");

    while RUNNING.load(Ordering::SeqCst) {
        while let Some(sample) = subscriber.receive()? {
            let len = sample.len as usize;
            if len > POSE_BUF {
                eprintln!("[sim_bridge] bad message len {len}");
                continue;
            }
            let reader = capnp::serialize::read_message(
                &sample.data[..len],
                capnp::message::ReaderOptions::default(),
            );
            let reader = match reader {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("[sim_bridge] capnp read error: {e}");
                    continue;
                }
            };
            let pose = match reader.get_root::<sim_capnp::camera_pose::Reader>() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("[sim_bridge] capnp root error: {e}");
                    continue;
                }
            };

            let c = CameraPoseC {
                x: pose.get_x(),
                y: pose.get_y(),
                z: pose.get_z(),
                qw: pose.get_qw(),
                qx: pose.get_qx(),
                qy: pose.get_qy(),
                qz: pose.get_qz(),
                timestamp_us: pose.get_timestamp_us(),
            };

            if let Ok(mut s) = LATEST.lock() {
                s.pose = c;
                s.has_new = true;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    eprintln!("[sim_bridge] shutting down");
    Ok(())
}

// ── Exported C API ───────────────────────────────────────────────

/// Start the background subscriber thread.  Returns `true` on success.
#[no_mangle]
pub extern "C" fn sim_bridge_init() -> bool {
    if RUNNING.load(Ordering::SeqCst) {
        return true; // already running
    }
    RUNNING.store(true, Ordering::SeqCst);

    thread::Builder::new()
        .name("sim_bridge".into())
        .spawn(|| {
            if let Err(e) = bridge_loop() {
                eprintln!("[sim_bridge] fatal: {e}");
                RUNNING.store(false, Ordering::SeqCst);
            }
        })
        .is_ok()
}

/// Copy the latest pose into `*out` if a new one has arrived since the last
/// call.  Returns `true` when `*out` was written.
#[no_mangle]
pub extern "C" fn sim_bridge_poll_pose(out: *mut CameraPoseC) -> bool {
    if out.is_null() {
        return false;
    }
    let mut s = match LATEST.lock() {
        Ok(s) => s,
        Err(_) => return false,
    };
    if s.has_new {
        unsafe {
            *out = s.pose;
        }
        s.has_new = false;
        true
    } else {
        false
    }
}

/// Stop the background thread and release resources.
#[no_mangle]
pub extern "C" fn sim_bridge_cleanup() {
    RUNNING.store(false, Ordering::SeqCst);
}
