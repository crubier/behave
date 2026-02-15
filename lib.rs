//! Behave -- behavior tree framework for autonomous drones.
//!
//! Library root. Re-exports modules from `robot/` (general infra)
//! and `actions/` (behavior tree action types).

// ── Robot general infrastructure ───────────────────────────────

#[path = "robot/controls.rs"]
pub mod controls;

#[path = "robot/ipc.rs"]
pub mod ipc;

#[path = "robot/logging.rs"]
pub mod logging;

#[path = "robot/topics/mod.rs"]
pub mod topics;

// ── Actions ────────────────────────────────────────────────────

#[path = "actions/mod.rs"]
pub mod actions;

// ── Generated FlatBuffer schemas ────────────────────────────────

#[allow(unused_imports, clippy::all, warnings)]
pub mod schema {
    include!(concat!(env!("OUT_DIR"), "/flatbuffers/mod.rs"));
}
