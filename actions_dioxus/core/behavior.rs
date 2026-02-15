//! Unified node behavior traits for behavior tree elements.
//!
//! Three layers:
//!
//! - [`ActionNode<A,O,R,S>`] -- generic struct holding the 4 proto fields.
//! - [`Behavior`] -- trait for lifecycle logic (activate, tick, child_complete).
//!   You only implement this. Data accessors are automatic.
//! - [`NodeBehavior`] -- raw-bytes trait used by the renderer.
//!   Automatically implemented via blanket impl. Never implement directly.

use prost::Message;

use crate::actions::io::ActionIO;

// ── Control flow types ──────────────────────────────────────────

/// What a node tells the renderer to do next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeResponse {
    Running,
    ActivateChild(usize),
    ActivateAllChildren,
    Success,
    Failure,
}

/// Outcome of a child completing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildResult {
    Success,
    Failure,
}

// ── ActionNode (generic struct) ─────────────────────────────────

/// Generic node struct that every action type uses.
///
/// Holds the 4 proto fields. Concrete types are type aliases:
///
/// ```ignore
/// type TakeoffNode = ActionNode<TakeoffArgs, TakeoffOutput, TakeoffResult, TakeoffState>;
/// ```
pub struct ActionNode<A, O, R, S> {
    pub args: A,
    pub output: O,
    pub result: R,
    pub state: S,
}

impl<A: Message + Default, O: Default, R: Default, S: Default> ActionNode<A, O, R, S> {
    /// Decode args from serialized protobuf, default everything else.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            args: A::decode(bytes).unwrap_or_default(),
            output: O::default(),
            result: R::default(),
            state: S::default(),
        }
    }
}

// ── Behavior (what you implement) ───────────────────────────────

/// Lifecycle trait for action nodes. Implement this on
/// `ActionNode<YourArgs, YourOutput, YourResult, YourState>`.
///
/// You get data accessors (output/result/state) and encoding for free.
/// Just write the behavior.
///
/// ```ignore
/// impl Behavior for TakeoffNode {
///     fn on_activate(&mut self, io: &ActionIO) -> NodeResponse {
///         controls::send_takeoff(io.cmd, self.args.altitude_m);
///         NodeResponse::Running
///     }
///     fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
///         self.output.current_altitude_m = io.sense_status.altitude_m;
///         if close_enough { NodeResponse::Success } else { NodeResponse::Running }
///     }
/// }
/// ```
pub trait Behavior {
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse;

    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        let _ = io;
        NodeResponse::Running
    }

    fn on_child_complete(
        &mut self,
        child_index: usize,
        child_count: usize,
        result: ChildResult,
    ) -> NodeResponse {
        let _ = (child_index, child_count);
        match result {
            ChildResult::Success => NodeResponse::Success,
            ChildResult::Failure => NodeResponse::Failure,
        }
    }

    fn on_command(&mut self, _cmd: &[u8]) {}
}

// ── NodeBehavior (raw bytes, used by renderer) ──────────────────

/// Low-level trait used by the renderer. Returns serialized protobuf.
///
/// **Do not implement this directly.** Implement [`Behavior`] on an
/// [`ActionNode`] instead -- you get this for free.
pub trait NodeBehavior {
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse;
    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse;
    fn on_child_complete(&mut self, child_index: usize, child_count: usize, result: ChildResult) -> NodeResponse;
    fn output_bytes(&self) -> Vec<u8>;
    fn result_bytes(&self) -> Vec<u8>;
    fn state_bytes(&self) -> Vec<u8>;
    fn on_command(&mut self, cmd: &[u8]);
}

/// Blanket impl: ActionNode<A,O,R,S> where Behavior is implemented
/// automatically becomes a NodeBehavior. Encoding + data accessors
/// are handled here -- zero boilerplate in node code.
impl<A, O, R, S> NodeBehavior for ActionNode<A, O, R, S>
where
    A: 'static,
    O: Message + Clone + Default + 'static,
    R: Message + Clone + Default + 'static,
    S: Message + Clone + Default + 'static,
    Self: Behavior,
{
    fn on_activate(&mut self, io: &ActionIO) -> NodeResponse {
        Behavior::on_activate(self, io)
    }
    fn on_tick(&mut self, io: &ActionIO) -> NodeResponse {
        Behavior::on_tick(self, io)
    }
    fn on_child_complete(&mut self, i: usize, n: usize, r: ChildResult) -> NodeResponse {
        Behavior::on_child_complete(self, i, n, r)
    }
    fn output_bytes(&self) -> Vec<u8> {
        self.output.encode_to_vec()
    }
    fn result_bytes(&self) -> Vec<u8> {
        self.result.encode_to_vec()
    }
    fn state_bytes(&self) -> Vec<u8> {
        self.state.encode_to_vec()
    }
    fn on_command(&mut self, cmd: &[u8]) {
        Behavior::on_command(self, cmd)
    }
}
