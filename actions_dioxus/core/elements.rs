//! Custom Dioxus element definitions for behavior tree nodes.
//!
//! The `rsx!` macro resolves elements through `dioxus_elements::elements::TAG`
//! and `dioxus_elements::TAG`. This module provides both paths.
//!
//! # Usage
//!
//! ```ignore
//! use dioxus::prelude::*;
//! use behave::actions_dioxus::elements as dioxus_elements;
//!
//! rsx! {
//!     sequence {
//!         takeoff { altitude_m: 50.0 }
//!         take_photo {}
//!         land { descent_speed_ms: 2.0 }
//!     }
//! }
//! ```

/// Attribute descriptor type used by the `rsx!` macro.
pub type AttributeDescription = (&'static str, Option<&'static str>, bool);

/// Namespace for all behavior tree elements.
const BT_NS: Option<&str> = Some("bt");

/// Macro to define a custom behavior tree element as a module.
macro_rules! bt_element {
    ($name:ident, [$( $attr:ident ),*]) => {
        #[allow(non_camel_case_types, non_upper_case_globals)]
        pub mod $name {
            pub const TAG_NAME: &'static str = stringify!($name);
            pub const NAME_SPACE: Option<&'static str> = super::BT_NS;

            $(
                pub const $attr: super::AttributeDescription =
                    (stringify!($attr), super::BT_NS, false);
            )*
        }
    };
}

/// The `elements` sub-module that the `rsx!` macro resolves through.
pub mod elements {
    use super::*;

    // ── Composite elements ──────────────────────────────────────

    bt_element!(sequence,       []);
    bt_element!(fallback,       []);
    bt_element!(parallel,       []);

    // ── Leaf elements ───────────────────────────────────────────

    bt_element!(takeoff,        [altitude_m]);
    bt_element!(land,           [descent_speed_ms]);
    bt_element!(goto,           [easting_m, northing_m, altitude_m, speed_ms]);
    bt_element!(home,           [altitude_m]);
    bt_element!(photo,          []);

    /// Completions module -- required by the `rsx!` macro for IDE support.
    pub mod completions {
        /// Marker type for brace-based element completions.
        #[allow(non_camel_case_types)]
        pub enum CompleteWithBraces {
            sequence,
            fallback,
            parallel,
            takeoff,
            land,
            goto_waypoint,
            return_home,
            take_photo,
        }
    }
}

// Re-export element modules at the top level so the macro can resolve
// `dioxus_elements::sequence` in addition to `dioxus_elements::elements::sequence`.
pub use elements::*;
