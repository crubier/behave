//! Core data types for the behavior tree runtime.
//!
//! All data flowing through the tree is serialized as protobuf bytes.
//! Each node defines its own `.proto` with Args, Output, Result, Input,
//! and State messages. The trait interface uses `Vec<u8>` (serialized
//! protobuf) so the renderer is type-agnostic. Consumers deserialize
//! using the known message type for each node (identified by `Run.tag`).

use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use dioxus_core::AttributeValue;

// ── ProtoBytes ──────────────────────────────────────────────────

/// Wrapper for serialized protobuf bytes that can be passed through
/// Dioxus element attributes via `AttributeValue::Any`.
///
/// Component wrappers encode their Args proto into this, and the
/// renderer extracts the bytes on activation.
#[derive(Clone, Debug, PartialEq)]
pub struct ProtoBytes(pub Vec<u8>);

impl dioxus_core::IntoAttributeValue for ProtoBytes {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Any(Rc::new(self))
    }
}

// ── Time ────────────────────────────────────────────────────────

/// Microseconds since Unix epoch.
pub type Utime = u64;

/// Current time in microseconds since Unix epoch.
pub fn now_utime() -> Utime {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u64
}

// ── Run ─────────────────────────────────────────────────────────

/// Unique identifier for a single activation of a node.
pub type RunId = u64;

/// Status of a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    Running,
    Succeeded,
    Failed,
}

/// A single execution record for a node.
///
/// Each time a node is activated, a new `Run` is created. It tracks
/// identity, timing, and protobuf-serialized data at each stage:
///
/// - `output` -- real-time telemetry (updated each tick)
/// - `result` -- final data (set on completion)
/// - `state` -- internal state (for debugging/persistence)
/// - `children` -- sub-runs for composites
///
/// Consumers deserialize `output`/`result`/`state` using the protobuf
/// message type that matches `tag` (e.g. tag "takeoff" → `TakeoffOutput`).
#[derive(Debug, Clone)]
pub struct Run {
    /// Unique run identifier.
    pub id: RunId,
    /// Element tag (e.g. "takeoff", "sequence").
    pub tag: &'static str,
    /// Microsecond timestamp when this run started.
    pub started_at: Utime,
    /// Microsecond timestamp when this run ended. `None` if still running.
    pub ended_at: Option<Utime>,
    /// Current status.
    pub status: RunStatus,
    /// Real-time output (serialized protobuf, updated each tick).
    pub output: Vec<u8>,
    /// Final result (serialized protobuf, set on completion).
    pub result: Vec<u8>,
    /// Internal state (serialized protobuf, for debugging).
    pub state: Vec<u8>,
    /// Child runs (for composites). Ordered by activation.
    pub children: Vec<Run>,
}

impl Run {
    /// Create a new running Run.
    pub fn new(id: RunId, tag: &'static str, started_at: Utime) -> Self {
        Self {
            id,
            tag,
            started_at,
            ended_at: None,
            status: RunStatus::Running,
            output: Vec::new(),
            result: Vec::new(),
            state: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Mark this run as completed.
    pub fn complete(&mut self, status: RunStatus, result: Vec<u8>, state: Vec<u8>, ended_at: Utime) {
        self.status = status;
        self.result = result;
        self.state = state;
        self.ended_at = Some(ended_at);
    }
}
