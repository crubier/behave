//! Behave node -- tick-based behavior tree executor.
//!
//! Fully action-agnostic: receives ActionArgs, delegates everything
//! to `behave::actions::from_capnp()` and `behave::actions::tick()`.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::actions::io::{ActionIO, ControlResponseSnapshot, ControlStatusSnapshot, SenseStatusSnapshot};
use behave::actions::Tick;
use behave::controls::CmdPublisher;
use behave::ipc::CmdMessage;
use behave::schema::actions::action_capnp::action_args;
use behave::schema::control_response_capnp::control_response;
use behave::schema::control_status_capnp::control_status;
use behave::schema::sense_status_capnp::sense_status;
use behave::topics;

struct Iox2CmdPublisher<'a> {
    inner: &'a topics::Pub<{ topics::control::request::BUF }>,
}

impl<'a> CmdPublisher for Iox2CmdPublisher<'a> {
    fn send_envelope(&self, envelope: CmdMessage) -> Result<()> {
        let sample = self.inner.loan_uninit()?;
        sample.write_payload(envelope).send()?;
        Ok(())
    }
}

pub fn run() -> Result<()> {
    behave::logging::init("Behave");
    info!("starting");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    // ── Subscribe to action requests ──────────────────────────
    let action_sub = topics::behave::request::subscribe(&node)?;
    info!("subscribed to {}", topics::behave::request::NAME);

    // ── Publish control requests ──────────────────────────────
    let cmd_pub = topics::control::request::publish(&node)?;
    info!("publishing {}", topics::control::request::NAME);

    // ── Subscribe to topics exposed to actions ────────────────
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
        // ── Poll incoming action requests ────────────────────
        while let Some(typed) = topics::receive::<{ topics::behave::request::BUF }, action_args::Owned>(&action_sub)? {
            let args = typed.get()?;
            let id = args.get_id();
            let name = args.get_name()?.to_str().unwrap_or("?");
            info!("=== action #{id} \"{name}\" received ===");

            let root = behave::actions::from_capnp(&args)?;
            active_action = Some((id, root));
            info!("=== action #{id} ready ===");
        }

        // ── Poll control response ────────────────────────────
        while let Some(typed) = topics::receive::<{ topics::control::response::BUF }, control_response::Owned>(&ctrl_resp_sub)? {
            let r = typed.get()?;
            snap_ctrl_resp = ControlResponseSnapshot {
                command_id: r.get_command_id(),
                success: r.get_success(),
                message: r.get_message()?.to_str().unwrap_or("").to_string(),
            };
        }

        // ── Poll control status ──────────────────────────────
        while let Some(typed) = topics::receive::<{ topics::control::status::BUF }, control_status::Owned>(&ctrl_status_sub)? {
            let s = typed.get()?;
            snap_ctrl_status = ControlStatusSnapshot {
                armed: s.get_armed(),
                mode: s.get_mode().map_or(0, |m| m as u16),
                battery_pct: s.get_battery_pct(),
            };
        }

        // ── Poll sense status ────────────────────────────────
        while let Some(typed) = topics::receive::<{ topics::sense::status::BUF }, sense_status::Owned>(&sense_sub)? {
            let s = typed.get()?;
            snap_sense = SenseStatusSnapshot {
                easting_m: s.get_easting_m(),
                northing_m: s.get_northing_m(),
                altitude_m: s.get_altitude_m(),
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
