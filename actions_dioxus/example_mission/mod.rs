//! Example missions built from core elements and reusable components.

use dioxus::prelude::*;

#[allow(non_snake_case, unused)]
mod dioxus_elements {
    pub use crate::actions_dioxus::core::elements::*;
}

// ── Components ──────────────────────────────────────────────────

#[component]
fn SurveyWaypoint(easting_m: f64, northing_m: f64, altitude_m: f64) -> Element {
    rsx! {
        sequence {
            goto { easting_m, northing_m, altitude_m, speed_ms: 15.0 }
            photo {}
        }
    }
}

#[component]
fn MissionEnvelope(altitude_m: f64, children: Element) -> Element {
    rsx! {
        sequence {
            takeoff { altitude_m }
            {children}
            home { altitude_m }
            land { descent_speed_ms: 2.0 }
        }
    }
}

#[component]
pub fn ExampleMission() -> Element {
    rsx! {
        MissionEnvelope { altitude_m: 50.0,
            SurveyWaypoint { easting_m: 500.0, northing_m: 300.0, altitude_m: 80.0 }
            SurveyWaypoint { easting_m: 1200.0, northing_m: -150.0, altitude_m: 80.0 }
        }
    }
}

#[component]
pub fn FallbackMission() -> Element {
    rsx! {
        MissionEnvelope { altitude_m: 50.0,
            fallback {
                SurveyWaypoint { easting_m: 500.0, northing_m: 300.0, altitude_m: 80.0 }
                SurveyWaypoint { easting_m: 200.0, northing_m: 100.0, altitude_m: 60.0 }
            }
        }
    }
}

#[component]
pub fn ParallelMission() -> Element {
    rsx! {
        MissionEnvelope { altitude_m: 50.0,
            parallel {
                SurveyWaypoint { easting_m: 500.0, northing_m: 300.0, altitude_m: 80.0 }
                SurveyWaypoint { easting_m: 1200.0, northing_m: -150.0, altitude_m: 80.0 }
            }
        }
    }
}
