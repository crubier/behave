//! Control Mock node -- instantly executes commands.
//!
//! When a ControlRequest arrives, immediately publishes a SimRequest
//! with the target pose, acks the command, and updates ControlStatus.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::topics;
use behave::topics::control::request::*;
use behave::topics::control::response::ControlResponse;
use behave::topics::control::status::*;
use behave::topics::sim::CameraPose;
use behave::topics::sim::request::SimRequest;

fn now_us() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64
}

pub fn run() -> Result<()> {
    behave::logging::init("CtrlMock");
    info!("starting (mock -- instant command execution)");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let cmd_sub = topics::control::request::subscribe(&node)?;
    let ack_pub = topics::control::response::publish(&node)?;
    let ctrl_pub = topics::control::status::publish(&node)?;
    let sim_pub = topics::sim::request::publish(&node)?;

    let mut armed = false;
    let mut mode: u8 = MODE_IDLE;
    let mut easting = 0.0_f64;
    let mut northing = 0.0_f64;
    let mut altitude = 0.0_f64;

    info!("ready");

    while node.wait(Duration::from_millis(100)).is_ok() {
        while let Some(cmd) = topics::receive_native(&cmd_sub)? {
            let cmd_id = cmd.id;

            match cmd.cmd {
                CMD_ARM => { armed = true; mode = MODE_IDLE; info!("cmd #{cmd_id}: ARM"); }
                CMD_DISARM => { armed = false; mode = MODE_IDLE; info!("cmd #{cmd_id}: DISARM"); }
                CMD_TAKEOFF => { altitude = cmd.altitude_m; mode = MODE_HOVERING; info!("cmd #{cmd_id}: TAKEOFF -> alt={altitude:.1}m"); }
                CMD_LAND => { altitude = 0.0; mode = MODE_IDLE; info!("cmd #{cmd_id}: LAND -> touchdown"); }
                CMD_HOVER => { mode = MODE_HOVERING; info!("cmd #{cmd_id}: HOVER"); }
                CMD_RETURN_HOME => { easting = 0.0; northing = 0.0; mode = MODE_IDLE; info!("cmd #{cmd_id}: RTH -> home"); }
                CMD_GOTO => {
                    easting = cmd.easting_m; northing = cmd.northing_m; altitude = cmd.altitude_m;
                    mode = MODE_HOVERING;
                    info!("cmd #{cmd_id}: GOTO -> ({easting:.1}, {northing:.1}) alt={altitude:.1}m");
                }
                CMD_TRIGGER_CAMERA => { info!("cmd #{cmd_id}: CAMERA \"{}\"", cmd.camera_tag()); }
                _ => { warn!("cmd #{cmd_id}: unknown cmd {}", cmd.cmd); }
            }

            // Ack
            topics::publish(&ack_pub, ControlResponse::new(cmd_id, true, "ok"))?;

            // Publish SimRequest with the target pose
            topics::publish(&sim_pub, SimRequest {
                pose: CameraPose { x: easting, y: northing, z: altitude, qw: 1.0, ..Default::default() },
                utime: now_us(),
            })?;
        }

        // Publish control status every tick
        topics::publish(&ctrl_pub, ControlStatus { armed, mode, battery_pct: 100.0 })?;
    }

    warn!("node loop exited");
    Ok(())
}
