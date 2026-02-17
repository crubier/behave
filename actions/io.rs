//! ActionAPI -- the robot interface exposed to actions.
//!
//! Actions can:
//!   - Send:    ControlRequest (via `controls::send_*` helpers)
//!   - Receive: ControlResponse, ControlStatus, SenseStatus
//!   - Emit:    ActionOutputEnvelope, ActionResultEnvelope (via macros)
//!
//! Actions CANNOT access sim topics (SimRequest / SimStatus).

use std::sync::mpsc;

use crate::actions::{ActionOutputEnvelope, ActionResultEnvelope};
use crate::controls::CmdPublisher;
use crate::topics::control::response::ControlResponse;
use crate::topics::control::status::ControlStatus;
use crate::topics::sense::status::SenseStatus;

/// The interface exposed to actions for interacting with the robot.
///
/// Created by the behave node each tick. Actions receive a reference
/// to this -- they can send control requests, read the latest state,
/// and emit results/outputs through the notification channels.
pub struct ActionAPI<'a> {
    /// Send control requests (arm, takeoff, goto, etc.)
    pub cmd: &'a dyn CmdPublisher,

    /// Latest control response (ack from flight controller)
    pub control_response: ControlResponse,

    /// Latest control status (armed, mode, battery)
    pub control_status: ControlStatus,

    /// Latest sense status (position)
    pub sense_status: SenseStatus,

    /// Channel for streaming result notifications (set_result! sends here).
    pub result_tx: &'a mpsc::Sender<ActionResultEnvelope>,

    /// Channel for streaming output notifications (push_output! sends here).
    pub output_tx: &'a mpsc::Sender<ActionOutputEnvelope>,
}
