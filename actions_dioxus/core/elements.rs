//! Custom Dioxus element definitions for behavior tree nodes.
//!
//! Each element has a single `args` attribute that carries serialized
//! protobuf Args via `ProtoBytes`. Component wrappers provide the
//! typed interface; bare elements are an implementation detail.

#![allow(non_upper_case_globals)]

/// Attribute descriptor type used by the `rsx!` macro.
pub type AttributeDescription = (&'static str, Option<&'static str>, bool);

const BT_NS: Option<&str> = Some("bt");

macro_rules! bt_element {
    ($name:ident) => {
        #[allow(non_camel_case_types, non_upper_case_globals)]
        pub mod $name {
            pub const TAG_NAME: &'static str = stringify!($name);
            pub const NAME_SPACE: Option<&'static str> = super::BT_NS;
            pub const args: super::AttributeDescription = ("args", super::BT_NS, false);
        }
    };
}

pub mod elements {
    use super::*;

    bt_element!(sequence);
    bt_element!(fallback);
    bt_element!(parallel);
    bt_element!(takeoff);
    bt_element!(land);
    bt_element!(goto);
    bt_element!(home);
    bt_element!(photo);

    pub mod completions {
        #[allow(non_camel_case_types)]
        pub enum CompleteWithBraces {
            sequence, fallback, parallel,
            takeoff, land, goto, home, photo,
        }
    }
}

pub use elements::*;
