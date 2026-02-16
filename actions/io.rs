//! ActionAPI -- the robot interface exposed to actions.
//!
//! Actions can:
//!   - Send:    ControlRequest (via `controls::send_*` helpers)
//!   - Receive: ControlResponse, ControlStatus, SenseStatus
//!
//! Actions CANNOT access sim topics (SimRequest / SimStatus).

use crate::controls::CmdPublisher;
use crate::topics::control::response::ControlResponse;
use crate::topics::control::status::ControlStatus;
use crate::topics::sense::status::SenseStatus;

/// The interface exposed to actions for interacting with the robot.
///
/// Created and updated by the behave node each tick.
/// Actions receive a reference to this -- they can send control
/// requests and read the latest state, but nothing else.
pub struct ActionAPI<'a> {
    /// Send control requests (arm, takeoff, goto, etc.)
    pub cmd: &'a dyn CmdPublisher,

    /// Latest control response (ack from flight controller)
    pub control_response: ControlResponse,

    /// Latest control status (armed, mode, battery)
    pub control_status: ControlStatus,

    /// Latest sense status (position)
    pub sense_status: SenseStatus,
}
