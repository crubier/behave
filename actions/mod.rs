//! Action tree runtime -- ActionNode and top-level dispatch.
//!
//! Uses generated protobuf types from prost. Each action folder
//! has a `.proto` schema and a `pub mod proto` with the generated code.

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
use prost::Message;

pub use io::ActionIO;

// Top-level composed proto types (ActionArgs, ActionNode, ActionResult)
pub mod action_proto {
    include!(concat!(env!("OUT_DIR"), "/behave.actions.rs"));
}

use action_proto::{action_args, ActionNode as ProtoActionNode};
use takeoff::proto::TakeoffResult;
use land::proto::LandResult;
use goto_waypoint::proto::GotoWaypointResult;
use return_home::proto::ReturnHomeResult;
use take_photo::proto::TakePhotoResult;
use sequence::proto::SequenceResult;
use fallback::proto::FallbackResult;

// ── Tick result type ───────────────────────────────────────────

pub enum Tick<O, R> {
    Running(O),
    Success(R),
    Failure(R),
}

// ── Action variant (wraps prost-generated structs) ──────────────

pub enum ActionArgsKind {
    Sequence(sequence::proto::SequenceArgs),
    Fallback(fallback::proto::FallbackArgs),
    Takeoff(takeoff::proto::TakeoffArgs),
    Land(land::proto::LandArgs),
    GotoWaypoint(goto_waypoint::proto::GotoWaypointArgs),
    ReturnHome(return_home::proto::ReturnHomeArgs),
    TakePhoto(take_photo::proto::TakePhotoArgs),
}

// ── Action result (wraps prost-generated structs) ───────────────

pub enum ActionResultKind {
    Sequence(SequenceResult),
    Fallback(FallbackResult),
    Takeoff(TakeoffResult),
    Land(LandResult),
    GotoWaypoint(GotoWaypointResult),
    ReturnHome(ReturnHomeResult),
    TakePhoto(TakePhotoResult),
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

// ── Parse protobuf into ActionNode ─────────────────────────────

pub fn from_proto(pb: &ProtoActionNode) -> Result<ActionNode> {
    let id = pb.id;

    let args = pb.args.as_ref().ok_or_else(|| anyhow::anyhow!("missing args for node #{id}"))?;
    let action = args.action.as_ref().ok_or_else(|| anyhow::anyhow!("missing action variant for node #{id}"))?;

    let kind = match action {
        action_args::Action::Sequence(a) => ActionArgsKind::Sequence(a.clone()),
        action_args::Action::Fallback(a) => ActionArgsKind::Fallback(a.clone()),
        action_args::Action::Takeoff(a) => ActionArgsKind::Takeoff(a.clone()),
        action_args::Action::Land(a) => ActionArgsKind::Land(a.clone()),
        action_args::Action::GotoWaypoint(a) => ActionArgsKind::GotoWaypoint(a.clone()),
        action_args::Action::ReturnHome(a) => ActionArgsKind::ReturnHome(a.clone()),
        action_args::Action::TakePhoto(a) => ActionArgsKind::TakePhoto(a.clone()),
    };

    let children = pb.children.iter()
        .map(|child| from_proto(child))
        .collect::<Result<Vec<_>>>()?;

    info!("built action #{id} ({} children)", children.len());

    Ok(ActionNode { id, kind, started: false, current_index: 0, children })
}

/// Decode a serialized protobuf ActionNode from bytes.
pub fn from_bytes(bytes: &[u8]) -> Result<ActionNode> {
    let pb = ProtoActionNode::decode(bytes)?;
    from_proto(&pb)
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
