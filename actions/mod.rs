//! Action tree runtime -- ActionNode and top-level dispatch.
//!
//! Uses generated FlatBuffer *ArgsArgs structs directly as owned data.
//! No hand-written action types -- the .fbs schema is the single source of truth.

pub mod io;
pub mod sequence;
pub mod fallback;
pub mod takeoff;
pub mod goto_waypoint;
pub mod return_home;
pub mod land;
pub mod take_photo;

use std::fmt;

use anyhow::{bail, Result};
use log::info;

pub use io::ActionIO;

use crate::schema::behave::actions::{
    ActionArgs as FbActionArgs,
    ActionArgsType,
    TakeoffArgsArgs, LandArgsArgs, GotoWaypointArgsArgs,
    ReturnHomeArgsArgs, TakePhotoArgsArgs, SequenceArgsArgs, FallbackArgsArgs,
    TakeoffResultArgs, LandResultArgs, GotoWaypointResultArgs,
    ReturnHomeResultArgs, TakePhotoResultArgs, SequenceResultArgs, FallbackResultArgs,
};

// ── Tick result type ───────────────────────────────────────────

pub enum Tick<O, R> {
    Running(O),
    Success(R),
    Failure(R),
}

// ── Action variant (wraps generated ArgsArgs structs) ──────────

pub enum ActionArgsKind {
    Sequence(SequenceArgsArgs),
    Fallback(FallbackArgsArgs),
    Takeoff(TakeoffArgsArgs),
    Land(LandArgsArgs),
    GotoWaypoint(GotoWaypointArgsArgs),
    ReturnHome(ReturnHomeArgsArgs),
    TakePhoto(TakePhotoArgsArgs),
}

// ── Action result (wraps generated ResultArgs structs) ─────────

pub enum ActionResultKind {
    Sequence(SequenceResultArgs),
    Fallback(FallbackResultArgs),
    Takeoff(TakeoffResultArgs),
    Land(LandResultArgs),
    GotoWaypoint(GotoWaypointResultArgs),
    ReturnHome(ReturnHomeResultArgs),
    TakePhoto(TakePhotoResultArgs),
}

impl fmt::Debug for ActionResultKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sequence(r) => f.debug_struct("Sequence")
                .field("success", &r.success)
                .field("children_completed", &r.children_completed)
                .field("failed_at_index", &r.failed_at_index)
                .finish(),
            Self::Fallback(r) => f.debug_struct("Fallback")
                .field("success", &r.success)
                .field("succeeded_at_index", &r.succeeded_at_index)
                .field("children_attempted", &r.children_attempted)
                .finish(),
            Self::Takeoff(r) => f.debug_struct("Takeoff")
                .field("reached_altitude_m", &r.reached_altitude_m)
                .field("success", &r.success)
                .finish(),
            Self::Land(r) => f.debug_struct("Land")
                .field("success", &r.success)
                .finish(),
            Self::GotoWaypoint(r) => f.debug_struct("GotoWaypoint")
                .field("final_easting_m", &r.final_easting_m)
                .field("final_northing_m", &r.final_northing_m)
                .field("final_altitude_m", &r.final_altitude_m)
                .field("success", &r.success)
                .finish(),
            Self::ReturnHome(r) => f.debug_struct("ReturnHome")
                .field("success", &r.success)
                .finish(),
            Self::TakePhoto(r) => f.debug_struct("TakePhoto")
                .field("success", &r.success)
                .finish(),
        }
    }
}

/// Runtime node in the behavior tree.
pub struct ActionNode {
    pub id: u64,
    pub kind: ActionArgsKind,
    pub started: bool,
    pub current_index: usize,
    pub children: Vec<ActionNode>,
}

// ── Parse FlatBuffer into ActionNode ───────────────────────────

pub fn from_flatbuf(fb: &FbActionArgs<'_>) -> Result<ActionNode> {
    let id = fb.id();

    let kind = match fb.action_type() {
        ActionArgsType::SequenceArgs => ActionArgsKind::Sequence(SequenceArgsArgs {}),
        ActionArgsType::FallbackArgs => ActionArgsKind::Fallback(FallbackArgsArgs {}),
        ActionArgsType::TakeoffArgs => {
            let a = fb.action_as_takeoff_args().unwrap();
            ActionArgsKind::Takeoff(TakeoffArgsArgs { altitude_m: a.altitude_m() })
        }
        ActionArgsType::LandArgs => {
            let a = fb.action_as_land_args().unwrap();
            ActionArgsKind::Land(LandArgsArgs { descent_speed_ms: a.descent_speed_ms() })
        }
        ActionArgsType::GotoWaypointArgs => {
            let a = fb.action_as_goto_waypoint_args().unwrap();
            ActionArgsKind::GotoWaypoint(GotoWaypointArgsArgs {
                easting_m: a.easting_m(), northing_m: a.northing_m(),
                altitude_m: a.altitude_m(), speed_ms: a.speed_ms(),
            })
        }
        ActionArgsType::ReturnHomeArgs => {
            let a = fb.action_as_return_home_args().unwrap();
            ActionArgsKind::ReturnHome(ReturnHomeArgsArgs { altitude_m: a.altitude_m() })
        }
        ActionArgsType::TakePhotoArgs => ActionArgsKind::TakePhoto(TakePhotoArgsArgs {}),
        _ => bail!("unknown action type for node #{id}"),
    };

    let children = if let Some(kids) = fb.children() {
        (0..kids.len()).map(|i| from_flatbuf(&kids.get(i))).collect::<Result<Vec<_>>>()?
    } else {
        Vec::new()
    };

    info!("built action #{id} ({} children)", children.len());

    Ok(ActionNode { id, kind, started: false, current_index: 0, children })
}

// ── Start / Tick dispatch ──────────────────────────────────────

fn start_node(node: &mut ActionNode, io: &ActionIO) {
    if node.started { return; }
    node.started = true;

    match &node.kind {
        ActionArgsKind::Sequence(_) => sequence::start(node),
        ActionArgsKind::Fallback(_) => fallback::start(node),
        ActionArgsKind::Takeoff(_) => takeoff::start(node, io),
        ActionArgsKind::GotoWaypoint(_) => goto_waypoint::start(node, io),
        ActionArgsKind::ReturnHome(_) => return_home::start(node, io),
        ActionArgsKind::Land(_) => land::start(node, io),
        ActionArgsKind::TakePhoto(_) => take_photo::start(node, io),
    }
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), ActionResultKind> {
    start_node(node, io);

    match &node.kind {
        ActionArgsKind::Sequence(_) => sequence::tick(node, io),
        ActionArgsKind::Fallback(_) => fallback::tick(node, io),
        ActionArgsKind::Takeoff(_) => takeoff::tick(node, io),
        ActionArgsKind::GotoWaypoint(_) => goto_waypoint::tick(node, io),
        ActionArgsKind::ReturnHome(_) => return_home::tick(node, io),
        ActionArgsKind::Land(_) => land::tick(node, io),
        ActionArgsKind::TakePhoto(_) => take_photo::tick(node, io),
    }
}
