//! Behave node -- tick-based behavior tree executor.
//!
//! Receives protobuf ActionArgs, creates an ActionRun, and ticks it.
//! Routes ActionInputs to the right runs, streams ActionResults and
//! ActionOutputs back (via mpsc channels in ActionAPI, no tree-walking).

use std::sync::mpsc;
use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};
use prost::Message;

use behave::actions::io::ActionAPI;
use behave::actions::{
    ActionInputEnvelope, ActionOutputEnvelope, ActionResultEnvelope, ActionRun, RunStatus,
    TickResult,
};
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

/// Config flags (can be driven by env vars later).
struct BehaveConfig {
    /// Publish individual ActionOutputEnvelopes as they happen.
    pub stream_outputs: bool,
    /// Publish the full ActionRun tree each tick.
    pub publish_tree: bool,
}

impl Default for BehaveConfig {
    fn default() -> Self {
        Self {
            stream_outputs: true,
            publish_tree: true,
        }
    }
}

pub fn run() -> Result<()> {
    behave::logging::init("Behave");
    info!("starting");

    let config = BehaveConfig::default();

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    // ── Subscriptions ─────────────────────────────────────────
    let action_sub = topics::behave::request::subscribe(&node)?;
    info!("subscribed to {}", topics::behave::request::NAME);

    let input_sub = topics::behave::input::subscribe(&node)?;
    info!("subscribed to {}", topics::behave::input::NAME);

    let ctrl_resp_sub = topics::control::response::subscribe(&node)?;
    info!("subscribed to {}", topics::control::response::NAME);

    let ctrl_status_sub = topics::control::status::subscribe(&node)?;
    info!("subscribed to {}", topics::control::status::NAME);

    let sense_sub = topics::sense::status::subscribe(&node)?;
    info!("subscribed to {}", topics::sense::status::NAME);

    // ── Publishers ────────────────────────────────────────────
    let cmd_pub = topics::control::request::publish(&node)?;
    info!("publishing {}", topics::control::request::NAME);

    let state_pub = topics::behave::state::publish(&node)?;
    info!("publishing {}", topics::behave::state::NAME);

    let result_pub = topics::behave::result::publish(&node)?;
    info!("publishing {}", topics::behave::result::NAME);

    let output_pub = topics::behave::output::publish(&node)?;
    info!("publishing {}", topics::behave::output::NAME);

    let publisher = Iox2CmdPublisher { inner: &cmd_pub };

    // ── Notification channels (actions -> behave node) ────────
    let (result_tx, result_rx) = mpsc::channel::<ActionResultEnvelope>();
    let (output_tx, output_rx) = mpsc::channel::<ActionOutputEnvelope>();

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

        // ── Poll action inputs ────────────────────────────────
        if let Some(ref mut run) = active_run {
            while let Some(sample) = input_sub.receive()? {
                let bytes = &sample.data[..sample.len as usize];
                match ActionInputEnvelope::decode(bytes) {
                    Ok(envelope) => {
                        info!("routing input to tree (target: {:?})", envelope.target);
                        behave::actions::route_input(run, &envelope);
                    }
                    Err(e) => warn!("failed to parse action input: {e}"),
                }
            }
        }

        // ── Poll native topics ───────────────────────────────
        while let Some(r) = topics::receive_native(&ctrl_resp_sub)? { snap_resp = r; }
        while let Some(s) = topics::receive_native(&ctrl_status_sub)? { snap_ctrl = s; }
        while let Some(s) = topics::receive_native(&sense_sub)? { snap_sense = s; }

        // ── Tick active run ──────────────────────────────────
        if let Some(ref mut run) = active_run {
            let api = ActionAPI {
                cmd: &publisher,
                control_response: snap_resp,
                control_status: snap_ctrl,
                sense_status: snap_sense,
                result_tx: &result_tx,
                output_tx: &output_tx,
            };

            let result = behave::actions::tick(&api, run);

            // ── Publish full tree state ──────────────────────
            if config.publish_tree {
                let bytes = behave::actions::to_bytes(run);
                if let Err(e) = topics::publish_bytes(&state_pub, &bytes) {
                    warn!("failed to publish mission state: {e}");
                }
            }

            // ── Drain result notifications ───────────────────
            while let Ok(envelope) = result_rx.try_recv() {
                let bytes = envelope.encode_to_vec();
                if let Err(e) = topics::publish_bytes(&result_pub, &bytes) {
                    warn!("failed to publish result: {e}");
                }
            }

            // ── Drain output notifications ───────────────────
            if config.stream_outputs {
                while let Ok(envelope) = output_rx.try_recv() {
                    let bytes = envelope.encode_to_vec();
                    if let Err(e) = topics::publish_bytes(&output_pub, &bytes) {
                        warn!("failed to publish output: {e}");
                    }
                }
            }

            match result {
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
