//! Behave node -- tick-based behavior tree executor.
//!
//! Fully action-agnostic: receives ActionArgs (capnp), delegates everything
//! to `behave::actions::from_capnp()` and `behave::actions::tick()`.
//! Topic I/O uses native iceoryx2 structs (no capnp).

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::actions::io::{ActionIO, ControlResponseSnapshot, ControlStatusSnapshot, SenseStatusSnapshot};
use behave::actions::Tick;
use behave::controls::CmdPublisher;
use behave::schema::actions::action_capnp::action_args;
use behave::topics;
use behave::topics::control::request::ControlRequest;

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

    // ── Subscribe to action requests (capnp, legacy) ─────────
    let action_sub = topics::behave::request::subscribe(&node)?;
    info!("subscribed to {}", topics::behave::request::NAME);

    // ── Publish control requests (native) ────────────────────
    let cmd_pub = topics::control::request::publish(&node)?;
    info!("publishing {}", topics::control::request::NAME);

    // ── Subscribe to native topics for ActionIO ──────────────
    let ctrl_resp_sub = topics::control::response::subscribe(&node)?;
    info!("subscribed to {}", topics::control::response::NAME);

    let ctrl_status_sub = topics::control::status::subscribe(&node)?;
    info!("subscribed to {}", topics::control::status::NAME);

    let sense_sub = topics::sense::status::subscribe(&node)?;
    info!("subscribed to {}", topics::sense::status::NAME);

    let publisher = Iox2CmdPublisher { inner: &cmd_pub };

    // ── Snapshots updated each tick ──────────────────────────
    let mut snap_ctrl_resp = ControlResponseSnapshot::default();
    let mut snap_ctrl_status = ControlStatusSnapshot::default();
    let mut snap_sense = SenseStatusSnapshot::default();

    info!("ready -- waiting for action requests");

    let mut active_action: Option<(u64, behave::actions::ActionNode)> = None;

    while node.wait(Duration::from_millis(100)).is_ok() {
        // ── Poll action requests (capnp) ─────────────────────
        while let Some(typed) = topics::receive::<{ topics::behave::request::BUF }, action_args::Owned>(&action_sub)? {
            let args = typed.get()?;
            let id = args.get_id();
            let name = args.get_name()?.to_str().unwrap_or("?");
            info!("=== action #{id} \"{name}\" received ===");

            let root = behave::actions::from_capnp(&args)?;
            active_action = Some((id, root));
            info!("=== action #{id} ready ===");
        }

        // ── Poll control response (native) ───────────────────
        while let Some(resp) = topics::receive_native(&ctrl_resp_sub)? {
            snap_ctrl_resp = ControlResponseSnapshot {
                command_id: resp.command_id,
                success: resp.success,
                message: resp.message().to_string(),
            };
        }

        // ── Poll control status (native) ─────────────────────
        while let Some(status) = topics::receive_native(&ctrl_status_sub)? {
            snap_ctrl_status = ControlStatusSnapshot {
                armed: status.armed,
                mode: status.mode as u16,
                battery_pct: status.battery_pct,
            };
        }

        // ── Poll sense status (native) ───────────────────────
        while let Some(sense) = topics::receive_native(&sense_sub)? {
            snap_sense = SenseStatusSnapshot {
                easting_m: sense.easting_m,
                northing_m: sense.northing_m,
                altitude_m: sense.altitude_m,
            };
        }

        // ── Tick active action ───────────────────────────────
        if let Some((id, ref mut root)) = active_action {
            let io = ActionIO {
                cmd: &publisher,
                control_response: snap_ctrl_resp.clone(),
                control_status: snap_ctrl_status.clone(),
                sense_status: snap_sense.clone(),
            };

            match behave::actions::tick(root, &io) {
                Tick::Running(()) => {}
                Tick::Success(_) => {
                    info!("=== action #{id} SUCCESS ===");
                    active_action = None;
                }
                Tick::Failure(_) => {
                    warn!("=== action #{id} FAILURE ===");
                    active_action = None;
                }
            }
        }
    }

    warn!("node loop exited");
    Ok(())
}
