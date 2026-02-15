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

// ── Generated Cap'n Proto schemas ──────────────────────────────

pub mod schema {
    // Re-export so cross-schema references (e.g. mission -> action) resolve.
    pub use self::actions::action_capnp;

    // ── Topic schemas (split per-topic) ────────────────────────
    pub mod control_command_capnp {
        include!(concat!(env!("OUT_DIR"), "/control_command_capnp.rs"));
    }
    pub mod control_ack_capnp {
        include!(concat!(env!("OUT_DIR"), "/control_ack_capnp.rs"));
    }
    pub mod drone_state_capnp {
        include!(concat!(env!("OUT_DIR"), "/drone_state_capnp.rs"));
    }
    pub mod sim_pose_capnp {
        include!(concat!(env!("OUT_DIR"), "/sim_pose_capnp.rs"));
    }
    pub mod sim_status_capnp {
        include!(concat!(env!("OUT_DIR"), "/sim_status_capnp.rs"));
    }

    // ── Action schemas ─────────────────────────────────────────
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
