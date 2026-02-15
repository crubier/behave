extern crate alloc;

use std::hash::{Hash, Hasher};

use iceoryx2::prelude::{SemanticString, ZeroCopySend};
use iceoryx2_bb_container::semantic_string;

const GREETING_TEXT_CAPACITY: usize = 64;

semantic_string! {
    /// Greeting text content sent between publisher and subscriber.
    name: GreetingText,
    capacity: GREETING_TEXT_CAPACITY,
    // No additional content/character constraints for now.
    invalid_content: |_string: &[u8]| false,
    invalid_characters: |_string: &[u8]| false,
    normalize: |this: &GreetingText| { this.clone() }
}

#[repr(C)]
#[derive(Clone, Debug, ZeroCopySend)]
pub struct Greeting {
    pub id: u64,
    pub text: GreetingText,
}

pub mod action;
pub mod controls;
pub mod ipc;
pub mod logging;

#[path = "../actions/mod.rs"]
pub mod actions;

pub mod schema {
    // Re-export so cross-schema references (e.g. mission -> action) resolve.
    // capnpc derives module paths from default_parent_module + file stem,
    // so mission_capnp.rs references crate::schema::action_capnp directly.
    pub use self::actions::action_capnp;

    pub mod messages_capnp {
        include!(concat!(env!("OUT_DIR"), "/messages_capnp.rs"));
    }
    pub mod controls_capnp {
        include!(concat!(env!("OUT_DIR"), "/controls_capnp.rs"));
    }
    pub mod mission_capnp {
        include!(concat!(env!("OUT_DIR"), "/mission_capnp.rs"));
    }
    pub mod sim_capnp {
        include!(concat!(env!("OUT_DIR"), "/sim_capnp.rs"));
    }
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
