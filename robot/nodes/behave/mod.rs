//! Behave node -- tick-based behavior tree executor.
//!
//! Receives protobuf ActionArgs, creates an ActionRun, and ticks it.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::actions::io::ActionIO;
use behave::actions::{ActionRun, RunStatus, TickResult};
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

    let mut active_run: Option<ActionRun> = None;

    while node.wait(Duration::from_millis(100)).is_ok() {
        // ── Poll action requests (protobuf in IpcMessage) ────
        while let Some(sample) = action_sub.receive()? {
            let bytes = &sample.data[..sample.len as usize];
            match behave::actions::from_bytes(bytes) {
                Ok(run) => {
                    info!("=== run #{} received ===", run.run_id);
                    active_run = Some(run);
                }
                Err(e) => warn!("failed to parse action tree: {e}"),
            }
        }

        // ── Poll native topics ───────────────────────────────
        while let Some(r) = topics::receive_native(&ctrl_resp_sub)? { snap_resp = r; }
        while let Some(s) = topics::receive_native(&ctrl_status_sub)? { snap_ctrl = s; }
        while let Some(s) = topics::receive_native(&sense_sub)? { snap_sense = s; }

        // ── Tick active run ──────────────────────────────────
        if let Some(ref mut run) = active_run {
            let io = ActionIO {
                cmd: &publisher,
                control_response: snap_resp,
                control_status: snap_ctrl,
                sense_status: snap_sense,
            };

            match behave::actions::tick(run, &io) {
                TickResult::Running => {}
                TickResult::Success => {
                    info!("=== run #{} SUCCESS: {:?} ===", run.run_id, run.result);
                    active_run = None;
                }
                TickResult::Failure => {
                    warn!("=== run #{} FAILURE: {:?} ===", run.run_id, run.result);
                    active_run = None;
                }
            }
        }
    }

    warn!("node loop exited");
    Ok(())
}
