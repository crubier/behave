//! Dioxus-based behavior tree renderer.
//!
//! This module provides a Dioxus custom renderer that executes behavior
//! trees. Every node type is a **core element** that implements the
//! unified [`core::NodeBehavior`] trait. Users can also create **user
//! elements** (Dioxus components) that compose core elements without
//! implementing any trait.
//!
//! # Architecture
//!
//! ```text
//!   ┌──────────────────────────────────┐
//!   │  User elements (components)      │
//!   │                                  │
//!   │  fn SurveyWaypoint() -> Element  │   Dioxus expands
//!   │    rsx! { sequence {             │   components into
//!   │      goto_waypoint { ... }       │ ──────────────────►  VirtualDom
//!   │      take_photo {}               │   core elements
//!   │    }}                            │
//!   └──────────────────────────────────┘
//!                                               │
//!                                      WriteMutations
//!                                               │
//!                                               ▼
//!   ┌──────────────────────────────────────────────────────┐
//!   │  BehaviorTreeRenderer                                │
//!   │                                                      │
//!   │  Core elements: sequence, fallback, takeoff, land,   │
//!   │  goto_waypoint, return_home, take_photo              │
//!   │                                                      │
//!   │  Each implements NodeBehavior:                        │
//!   │    on_activate  -> NodeResponse                      │
//!   │    on_tick       -> NodeResponse                     │
//!   │    on_child_complete -> NodeResponse                 │
//!   └──────────────────────────────────────────────────────┘
//! ```
//!
//! # Core elements
//!
//! Core elements implement [`core::NodeBehavior`] and are registered in
//! the renderer's factory. The composite/leaf distinction is emergent:
//!
//! - **Composites** (sequence, fallback) override `on_activate` and
//!   `on_child_complete` to manage children.
//! - **Leaves** (takeoff, land, ...) override `on_activate` and
//!   `on_tick` to execute actions.
//! - **Hybrids** can override all three (e.g. a while-decorator).
//!
//! # User elements (components)
//!
//! User elements compose core elements via standard Dioxus components.
//! No trait implementation or registration needed -- Dioxus expands them
//! into core elements before the renderer sees them.
//!
//! ```ignore
//! use dioxus_core::prelude::*;
//! use behave::actions_dioxus::elements as dioxus_elements;
//!
//! // User element: pure composition of core elements
//! #[component]
//! fn SurveyWaypoint(easting_m: f64, northing_m: f64, altitude_m: f64) -> Element {
//!     rsx! {
//!         sequence {
//!             goto_waypoint { easting_m, northing_m, altitude_m, speed_ms: 15.0 }
//!             take_photo {}
//!         }
//!     }
//! }
//!
//! // Mission using both core and user elements
//! fn mission() -> Element {
//!     rsx! {
//!         sequence {
//!             takeoff { altitude_m: 50.0 }
//!             SurveyWaypoint { easting_m: 500.0, northing_m: 300.0, altitude_m: 80.0 }
//!             SurveyWaypoint { easting_m: 1200.0, northing_m: -150.0, altitude_m: 80.0 }
//!             return_home { altitude_m: 60.0 }
//!             land { descent_speed_ms: 2.0 }
//!         }
//!     }
//! }
//! ```

pub mod core;
pub mod sequence;
pub mod fallback;
pub mod parallel;
pub mod takeoff;
pub mod land;
pub mod goto_waypoint;
pub mod return_home;
pub mod take_photo;
/// Top-level composed protobuf types: `ActionArgs`, `ActionResult`,
/// `ActionOutput`, `ActionInput`, `ActionState`, `ActionNode`, and `Run`.
pub mod action_proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.rs"));
}

pub mod from_proto;
pub mod example_mission;

/// Re-export the custom elements module for use as `dioxus_elements` in
/// consumer code.
///
/// ```ignore
/// use behave::actions_dioxus::elements as dioxus_elements;
/// ```
pub use self::core::elements;
