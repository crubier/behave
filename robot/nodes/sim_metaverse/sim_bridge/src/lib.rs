//! C FFI bridge: iceoryx2 camera-pose subscriber for UE5.
//!
//! A background thread subscribes to the `"behave/SimStatus"` iceoryx2 service
//! using the same `IpcMessage` type as the main behave crate (required for
//! iceoryx2 cross-process type matching). The UE5 game thread polls via the
//! exported C functions.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;

use behave::schema::sim_status_capnp::sim_status;
use behave::topics;

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
    pub utime: u64,
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
            utime: 0,
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
        utime: 0,
    },
    has_new: false,
});

static RUNNING: AtomicBool = AtomicBool::new(false);

// ── Background subscriber loop ──────────────────────────────────

fn bridge_loop() -> anyhow::Result<()> {
    use iceoryx2::prelude::*;

    let node = NodeBuilder::new()
        .name(&"sim_bridge".try_into()?)
        .create::<iceoryx2::prelude::ipc::Service>()?;

    let sub = topics::sim::status::subscribe(&node)?;

    eprintln!("[sim_bridge] subscriber ready on {}", topics::sim::status::NAME);

    while RUNNING.load(Ordering::SeqCst) {
        while let Some(typed) = topics::receive::<{ topics::sim::status::BUF }, sim_status::Owned>(&sub)? {
            let status = typed.get()?;
            let pose = status.get_pose()?;

            let c = CameraPoseC {
                x: pose.get_x(),
                y: pose.get_y(),
                z: pose.get_z(),
                qw: pose.get_qw(),
                qx: pose.get_qx(),
                qy: pose.get_qy(),
                qz: pose.get_qz(),
                utime: status.get_utime(),
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
