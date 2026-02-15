//! Action tree runtime -- ActionNode and top-level dispatch.
//!
//! This module is the only place that knows about individual action types.
//! The behave node just calls `from_flatbuf()` and `tick()`.

pub mod io;
pub mod sequence;
pub mod fallback;
pub mod takeoff;
pub mod goto_waypoint;
pub mod return_home;
pub mod land;
pub mod take_photo;

use anyhow::{bail, Result};
use log::info;

pub use io::ActionIO;

use crate::schema::behave::actions::{
    ActionArgs as FbActionArgs,
    ActionArgsType,
};

// ── Tick result type ───────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Tick<O, R> {
    Running(O),
    Success(R),
    Failure(R),
}

impl<O, R> Tick<O, R> {
    pub fn is_running(&self) -> bool { matches!(self, Tick::Running(_)) }
    pub fn is_done(&self) -> bool { !self.is_running() }
    pub fn is_success(&self) -> bool { matches!(self, Tick::Success(_)) }
    pub fn is_failure(&self) -> bool { matches!(self, Tick::Failure(_)) }
}

// ── Native action types ───────────────────────────────────────

/// The concrete action variant and its parameters.
#[derive(Debug, Clone)]
pub enum ActionArgs {
    Sequence,
    Fallback,
    Takeoff { altitude_m: f64 },
    Land { descent_speed_ms: f64 },
    GotoWaypoint { easting_m: f64, northing_m: f64, altitude_m: f64, speed_ms: f64 },
    ReturnHome { altitude_m: f64 },
    TakePhoto { tag: String },
}

/// Runtime node in the behavior tree.
pub struct ActionNode {
    pub id: u64,
    pub name: String,
    pub args: ActionArgs,
    pub started: bool,
    /// Current child index for sequence/fallback composites.
    pub current_index: usize,
    /// Child nodes (empty for leaves).
    pub children: Vec<ActionNode>,
}

// ── Parse FlatBuffer into native ActionNode ────────────────────

pub fn from_flatbuf(fb: &FbActionArgs<'_>) -> Result<ActionNode> {
    let id = fb.id();
    let name = fb.name().unwrap_or("(unnamed)").to_string();

    let args = match fb.action_type() {
        ActionArgsType::SequenceArgs => ActionArgs::Sequence,
        ActionArgsType::FallbackArgs => ActionArgs::Fallback,
        ActionArgsType::TakeoffArgs => {
            let a = fb.action_as_takeoff_args().unwrap();
            ActionArgs::Takeoff { altitude_m: a.altitude_m() }
        }
        ActionArgsType::LandArgs => {
            let a = fb.action_as_land_args().unwrap();
            ActionArgs::Land { descent_speed_ms: a.descent_speed_ms() }
        }
        ActionArgsType::GotoWaypointArgs => {
            let a = fb.action_as_goto_waypoint_args().unwrap();
            ActionArgs::GotoWaypoint {
                easting_m: a.easting_m(),
                northing_m: a.northing_m(),
                altitude_m: a.altitude_m(),
                speed_ms: a.speed_ms(),
            }
        }
        ActionArgsType::ReturnHomeArgs => {
            let a = fb.action_as_return_home_args().unwrap();
            ActionArgs::ReturnHome { altitude_m: a.altitude_m() }
        }
        ActionArgsType::TakePhotoArgs => {
            let a = fb.action_as_take_photo_args().unwrap();
            ActionArgs::TakePhoto { tag: a.tag().unwrap_or("").to_string() }
        }
        _ => bail!("unknown action type for node #{id}"),
    };

    let children = if let Some(kids) = fb.children() {
        let mut v = Vec::with_capacity(kids.len());
        for i in 0..kids.len() {
            v.push(from_flatbuf(&kids.get(i))?);
        }
        v
    } else {
        Vec::new()
    };

    info!("built {:?} \"{}\" (id={}, {} children)",
        std::mem::discriminant(&args), name, id, children.len());

    Ok(ActionNode {
        id, name, args,
        started: false,
        current_index: 0,
        children,
    })
}

// ── Start / Tick dispatch ──────────────────────────────────────

fn start_node(node: &mut ActionNode, io: &ActionIO) {
    if node.started { return; }
    node.started = true;

    match &node.args {
        ActionArgs::Sequence => sequence::start(node),
        ActionArgs::Fallback => fallback::start(node),
        ActionArgs::Takeoff { .. } => takeoff::start(node, io),
        ActionArgs::GotoWaypoint { .. } => goto_waypoint::start(node, io),
        ActionArgs::ReturnHome { .. } => return_home::start(node, io),
        ActionArgs::Land { .. } => land::start(node, io),
        ActionArgs::TakePhoto { .. } => take_photo::start(node, io),
    }
}

pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), bool> {
    start_node(node, io);

    match &node.args {
        ActionArgs::Sequence => sequence::tick(node, io),
        ActionArgs::Fallback => fallback::tick(node, io),
        ActionArgs::Takeoff { .. } => takeoff::tick(node, io),
        ActionArgs::GotoWaypoint { .. } => goto_waypoint::tick(node, io),
        ActionArgs::ReturnHome { .. } => return_home::tick(node, io),
        ActionArgs::Land { .. } => land::tick(node, io),
        ActionArgs::TakePhoto { .. } => take_photo::tick(node, io),
    }
}
