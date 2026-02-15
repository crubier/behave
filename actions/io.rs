//! ActionIO -- the subset of topics exposed to actions.
//!
//! Actions can:
//!   - Send:    ControlRequest (via `controls::send_*` helpers)
//!   - Receive: ControlResponse, ControlStatus, SenseStatus
//!
//! Actions CANNOT access sim topics (SimRequest / SimStatus).

use crate::controls::CmdPublisher;

// ── FlightMode constants (match control_status.capnp enum ordinals) ──

pub const MODE_IDLE: u16 = 0;
pub const MODE_TAKING_OFF: u16 = 1;
pub const MODE_FLYING: u16 = 2;
pub const MODE_LANDING: u16 = 3;
pub const MODE_HOVERING: u16 = 4;
pub const MODE_RETURNING: u16 = 5;

// ── Snapshots (plain Rust, no capnp in action code) ─────────

/// Latest control response received.
#[derive(Debug, Clone, Default)]
pub struct ControlResponseSnapshot {
    pub command_id: u64,
    pub success: bool,
    pub message: String,
}

/// Latest control status received.
#[derive(Debug, Clone, Default)]
pub struct ControlStatusSnapshot {
    pub armed: bool,
    pub mode: u16,
    pub battery_pct: f64,
}

/// Latest sense status received.
#[derive(Debug, Clone, Default)]
pub struct SenseStatusSnapshot {
    pub easting_m: f64,
    pub northing_m: f64,
    pub altitude_m: f64,
}

// ── ActionIO ────────────────────────────────────────────────

/// The interface exposed to actions for interacting with the robot.
///
/// Created and updated by the behave node each tick.
/// Actions receive a reference to this -- they can send control
/// requests and read the latest state, but nothing else.
pub struct ActionIO<'a> {
    /// Send control requests (arm, takeoff, goto, etc.)
    pub cmd: &'a dyn CmdPublisher,

    /// Latest control response (ack from flight controller)
    pub control_response: ControlResponseSnapshot,

    /// Latest control status (armed, mode, battery)
    pub control_status: ControlStatusSnapshot,

    /// Latest sense status (position)
    pub sense_status: SenseStatusSnapshot,
}
