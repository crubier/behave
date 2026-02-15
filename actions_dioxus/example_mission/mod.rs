//! Example missions built from component wrappers.

use dioxus::prelude::*;

use crate::actions_dioxus::takeoff::Takeoff;
use crate::actions_dioxus::land::Land;
use crate::actions_dioxus::goto_waypoint::Goto;
use crate::actions_dioxus::return_home::Home;
use crate::actions_dioxus::take_photo::Photo;
use crate::actions_dioxus::sequence::Sequence;
use crate::actions_dioxus::fallback::Fallback;
use crate::actions_dioxus::parallel::Parallel;

// ── Components ──────────────────────────────────────────────────

#[component]
fn SurveyWaypoint(easting_m: f64, northing_m: f64, altitude_m: f64) -> Element {
    rsx! {
        Sequence {
            Goto { easting_m, northing_m, altitude_m, speed_ms: 15.0 }
            Photo {}
        }
    }
}

#[component]
fn MissionEnvelope(altitude_m: f64, children: Element) -> Element {
    rsx! {
        Sequence {
            Takeoff { altitude_m }
            {children}
            Home { altitude_m }
            Land { descent_speed_ms: 2.0 }
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
            Fallback {
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
            Parallel {
                SurveyWaypoint { easting_m: 500.0, northing_m: 300.0, altitude_m: 80.0 }
                SurveyWaypoint { easting_m: 1200.0, northing_m: -150.0, altitude_m: 80.0 }
            }
        }
    }
}
