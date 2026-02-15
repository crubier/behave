//! Action tree runtime -- ActionNode and top-level dispatch.
//!
//! This module is the only place that knows about individual action types.
//! The behave node just calls `from_capnp()` and `tick()`.

pub mod io;
pub mod sequence;
pub mod fallback;
pub mod takeoff;
pub mod goto_waypoint;
pub mod return_home;
pub mod land;
pub mod take_photo;

use anyhow::Result;
use capnp::message::{Builder, HeapAllocator};
use log::info;

pub use io::ActionIO;
use crate::schema::actions::action_capnp::action_args;

// ── Tick result type ───────────────────────────────────────────

/// Result of a single tick of an action node.
#[derive(Debug, Clone)]
pub enum Tick<O, R> {
    /// Action is still running; here is the latest output.
    Running(O),
    /// Action completed successfully.
    Success(R),
    /// Action completed with failure.
    Failure(R),
}

impl<O, R> Tick<O, R> {
    pub fn is_running(&self) -> bool {
        matches!(self, Tick::Running(_))
    }
    pub fn is_done(&self) -> bool {
        !self.is_running()
    }
    pub fn is_success(&self) -> bool {
        matches!(self, Tick::Success(_))
    }
    pub fn is_failure(&self) -> bool {
        matches!(self, Tick::Failure(_))
    }
}

/// Which action variant this node represents.
#[derive(Debug, Clone, Copy)]
pub enum ActionVariant {
    Sequence,
    Fallback,
    Takeoff,
    GotoWaypoint,
    ReturnHome,
    Land,
    TakePhoto,
}

/// Runtime node in the behavior tree.
///
/// Stores capnp message builders for args and state (no native Rust mirrors).
/// The capnp schema is the single source of truth.
pub struct ActionNode {
    pub id: u64,
    pub name: String,
    pub variant: ActionVariant,
    pub started: bool,
    /// Original args (capnp message, immutable after creation).
    pub args_msg: Builder<HeapAllocator>,
    /// Mutable state (capnp message, mutated by tick).
    pub state_msg: Builder<HeapAllocator>,
    /// Child nodes (empty for leaves).
    pub children: Vec<ActionNode>,
}

/// Build an ActionNode tree from a capnp ActionArgs reader.
/// Does NOT call start -- composites start children lazily on first tick.
pub fn from_capnp(spec: &action_args::Reader<'_>) -> Result<ActionNode> {
    let id = spec.get_id();
    let name = spec.get_name()?.to_str().unwrap_or("(unnamed)").to_string();

    let (variant, args_msg) = match spec.which()? {
        action_args::Sequence(r) => {
            let mut msg = Builder::new_default();
            if let Ok(r) = r {
                msg.set_root(r)?;
            }
            (ActionVariant::Sequence, msg)
        }
        action_args::Fallback(r) => {
            let mut msg = Builder::new_default();
            if let Ok(r) = r {
                msg.set_root(r)?;
            }
            (ActionVariant::Fallback, msg)
        }
        action_args::Takeoff(r) => {
            let mut msg = Builder::new_default();
            msg.set_root(r?)?;
            (ActionVariant::Takeoff, msg)
        }
        action_args::GotoWaypoint(r) => {
            let mut msg = Builder::new_default();
            msg.set_root(r?)?;
            (ActionVariant::GotoWaypoint, msg)
        }
        action_args::ReturnHome(r) => {
            let mut msg = Builder::new_default();
            msg.set_root(r?)?;
            (ActionVariant::ReturnHome, msg)
        }
        action_args::Land(r) => {
            let mut msg = Builder::new_default();
            msg.set_root(r?)?;
            (ActionVariant::Land, msg)
        }
        action_args::TakePhoto(r) => {
            let mut msg = Builder::new_default();
            msg.set_root(r?)?;
            (ActionVariant::TakePhoto, msg)
        }
    };

    // Recursively build children
    let children_reader = spec.get_children()?;
    let mut children = Vec::with_capacity(children_reader.len() as usize);
    for i in 0..children_reader.len() {
        children.push(from_capnp(&children_reader.get(i))?);
    }

    info!("built {:?} \"{}\" (id={}, {} children)", variant, name, id, children.len());

    Ok(ActionNode {
        id,
        name,
        variant,
        started: false,
        args_msg,
        state_msg: Builder::new_default(),
        children,
    })
}

/// Start an action node (initialize state from args, may send commands).
fn start_node(node: &mut ActionNode, io: &ActionIO) {
    if node.started {
        return;
    }
    node.started = true;

    match node.variant {
        ActionVariant::Sequence => sequence::start(node),
        ActionVariant::Fallback => fallback::start(node),
        ActionVariant::Takeoff => takeoff::start(node, io),
        ActionVariant::GotoWaypoint => goto_waypoint::start(node, io),
        ActionVariant::ReturnHome => return_home::start(node, io),
        ActionVariant::Land => land::start(node, io),
        ActionVariant::TakePhoto => take_photo::start(node, io),
    }
}

/// Tick an action node. Starts it lazily if needed.
pub fn tick(node: &mut ActionNode, io: &ActionIO) -> Tick<(), bool> {
    start_node(node, io);

    match node.variant {
        ActionVariant::Sequence => sequence::tick(node, io),
        ActionVariant::Fallback => fallback::tick(node, io),
        ActionVariant::Takeoff => takeoff::tick(node, io),
        ActionVariant::GotoWaypoint => goto_waypoint::tick(node, io),
        ActionVariant::ReturnHome => return_home::tick(node, io),
        ActionVariant::Land => land::tick(node, io),
        ActionVariant::TakePhoto => take_photo::tick(node, io),
    }
}

/// Serialize the full state tree to bytes (for suspend).
pub fn suspend(node: &ActionNode) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    capnp::serialize::write_message(&mut buf, &node.state_msg)?;
    // TODO: recursively serialize children
    Ok(buf)
}
