//! Behave -- behavior tree framework for autonomous drones.
//!
//! Library root. Re-exports modules from `robot/` (general infra)
//! and `actions/` (behavior tree action types).

extern crate alloc;

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

// ── Generated Cap'n Proto schemas (actions only) ────────────────

pub mod schema {
    pub use self::actions::action_capnp;

    pub mod actions {
        pub mod action_capnp {
            include!(concat!(env!("OUT_DIR"), "/action_capnp.rs"));
        }
        pub mod sequence_capnp {
            include!(concat!(env!("OUT_DIR"), "/sequence/sequence_capnp.rs"));
        }
        pub mod fallback_capnp {
            include!(concat!(env!("OUT_DIR"), "/fallback/fallback_capnp.rs"));
        }
        pub mod takeoff_capnp {
            include!(concat!(env!("OUT_DIR"), "/takeoff/takeoff_capnp.rs"));
        }
        pub mod goto_waypoint_capnp {
            include!(concat!(env!("OUT_DIR"), "/goto_waypoint/goto_waypoint_capnp.rs"));
        }
        pub mod return_home_capnp {
            include!(concat!(env!("OUT_DIR"), "/return_home/return_home_capnp.rs"));
        }
        pub mod land_capnp {
            include!(concat!(env!("OUT_DIR"), "/land/land_capnp.rs"));
        }
        pub mod take_photo_capnp {
            include!(concat!(env!("OUT_DIR"), "/take_photo/take_photo_capnp.rs"));
        }
    }
}
