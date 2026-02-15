//! Behave node -- tick-based behavior tree executor.
//!
//! Receives protobuf ActionNode, parses into native Rust types,
//! then ticks the behavior tree using ActionIO.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::actions::io::ActionIO;
use behave::actions::Tick;
use behave::controls::CmdPublisher;
use behave::topics;
use behave::topics::control::request::ControlRequest;
use behave::topics::control::response::ControlResponse;
use behave::topics::control::status::ControlStatus;
use behave::topics::sense::status::SenseStatus;

struct Iox2CmdPublisher<'a> {
    inner: &'a topics::NativePub<ControlRequest>,
}

impl<'a> CmdPublisher for Iox2CmdPublisher<'a> {
    fn send_cmd(&self, cmd: ControlRequest) -> Result<()> {
        topics::publish(self.inner, cmd)
    }
}

pub fn run() -> Result<()> {
    behave::logging::init("Behave");
    info!("starting");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let action_sub = topics::behave::request::subscribe(&node)?;
    info!("subscribed to {}", topics::behave::request::NAME);

    let cmd_pub = topics::control::request::publish(&node)?;
    info!("publishing {}", topics::control::request::NAME);

    let ctrl_resp_sub = topics::control::response::subscribe(&node)?;
    info!("subscribed to {}", topics::control::response::NAME);

    let ctrl_status_sub = topics::control::status::subscribe(&node)?;
    info!("subscribed to {}", topics::control::status::NAME);

    let sense_sub = topics::sense::status::subscribe(&node)?;
    info!("subscribed to {}", topics::sense::status::NAME);

    let publisher = Iox2CmdPublisher { inner: &cmd_pub };

    let mut snap_resp = ControlResponse::default();
    let mut snap_ctrl = ControlStatus::default();
    let mut snap_sense = SenseStatus::default();

    info!("ready -- waiting for action requests");

    let mut active_action: Option<(u64, behave::actions::ActionNode)> = None;

    while node.wait(Duration::from_millis(100)).is_ok() {
        // ── Poll action requests (protobuf in IpcMessage) ────
        while let Some(sample) = action_sub.receive()? {
            let bytes = &sample.data[..sample.len as usize];
            match behave::actions::from_bytes(bytes) {
                Ok(root) => {
                    let id = root.id;
                    info!("=== action #{id} received ===");
                    active_action = Some((id, root));
                    info!("=== action #{id} ready ===");
                }
                Err(e) => warn!("failed to parse action tree: {e}"),
            }
        }

        // ── Poll native topics ───────────────────────────────
        while let Some(r) = topics::receive_native(&ctrl_resp_sub)? { snap_resp = r; }
        while let Some(s) = topics::receive_native(&ctrl_status_sub)? { snap_ctrl = s; }
        while let Some(s) = topics::receive_native(&sense_sub)? { snap_sense = s; }

        // ── Tick active action ───────────────────────────────
        if let Some((id, ref mut root)) = active_action {
            let io = ActionIO {
                cmd: &publisher,
                control_response: snap_resp,
                control_status: snap_ctrl,
                sense_status: snap_sense,
            };

            match behave::actions::tick(root, &io) {
                Tick::Running(()) => {}
                Tick::Success(result) => {
                    info!("=== action #{id} SUCCESS: {result:?} ===");
                    active_action = None;
                }
                Tick::Failure(result) => {
                    warn!("=== action #{id} FAILURE: {result:?} ===");
                    active_action = None;
                }
            }
        }
    }

    warn!("node loop exited");
    Ok(())
}
