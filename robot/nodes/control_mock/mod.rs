//! Control Mock node -- instantly executes commands.
//!
//! When a ControlRequest arrives, immediately publishes a SimRequest
//! with the target pose, acks the command, and updates ControlStatus.
//! The actual position flows through: SimRequest -> sim_mock -> SimStatus -> sense_mock -> SenseStatus.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::schema::control_request_capnp::control_request;
use behave::schema::control_status_capnp::FlightMode;
use behave::topics;

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
    let mut mode = FlightMode::Idle;
    let mut easting = 0.0_f64;
    let mut northing = 0.0_f64;
    let mut altitude = 0.0_f64;

    info!("ready");

    while node.wait(Duration::from_millis(100)).is_ok() {
        while let Some(typed) = topics::receive::<{ topics::control::request::BUF }, control_request::Owned>(&cmd_sub)? {
            let cmd = typed.get()?;
            let cmd_id = cmd.get_id();

            match cmd.which()? {
                control_request::Arm(()) => {
                    armed = true;
                    mode = FlightMode::Idle;
                    info!("cmd #{cmd_id}: ARM");
                }
                control_request::Disarm(()) => {
                    armed = false;
                    mode = FlightMode::Idle;
                    info!("cmd #{cmd_id}: DISARM");
                }
                control_request::Takeoff(r) => {
                    altitude = r?.get_altitude_m();
                    mode = FlightMode::Hovering;
                    info!("cmd #{cmd_id}: TAKEOFF -> alt={altitude:.1}m");
                }
                control_request::Land(_) => {
                    altitude = 0.0;
                    mode = FlightMode::Idle;
                    info!("cmd #{cmd_id}: LAND -> touchdown");
                }
                control_request::Hover(()) => {
                    mode = FlightMode::Hovering;
                    info!("cmd #{cmd_id}: HOVER");
                }
                control_request::ReturnHome(_) => {
                    easting = 0.0;
                    northing = 0.0;
                    mode = FlightMode::Idle;
                    info!("cmd #{cmd_id}: RTH -> home");
                }
                control_request::Goto(r) => {
                    let r = r?;
                    easting = r.get_easting_m();
                    northing = r.get_northing_m();
                    altitude = r.get_altitude_m();
                    mode = FlightMode::Hovering;
                    info!("cmd #{cmd_id}: GOTO -> ({easting:.1}, {northing:.1}) alt={altitude:.1}m");
                }
                control_request::TriggerCamera(r) => {
                    let tag = r?.get_tag()?.to_str()?;
                    info!("cmd #{cmd_id}: CAMERA \"{tag}\"");
                }
            }

            // Ack
            {
                let mut msg = capnp::message::Builder::new_default();
                {
                    let mut ack = msg.init_root::<behave::schema::control_response_capnp::control_response::Builder<'_>>();
                    ack.set_command_id(cmd_id);
                    ack.set_success(true);
                    ack.set_message("ok".into());
                }
                topics::control::response::send(&ack_pub, &msg)?;
            }

            // Publish SimRequest with the target pose
            // (sim_mock will echo it as SimStatus, sense_mock will forward as SenseStatus)
            {
                let mut msg = capnp::message::Builder::new_default();
                {
                    let mut req = msg.init_root::<behave::schema::sim_request_capnp::sim_request::Builder<'_>>();
                    req.set_utime(now_us());
                    let mut pose = req.init_pose();
                    pose.set_x(easting);
                    pose.set_y(northing);
                    pose.set_z(altitude);
                    pose.set_qw(1.0);
                    pose.set_qx(0.0);
                    pose.set_qy(0.0);
                    pose.set_qz(0.0);
                }
                topics::sim::request::send(&sim_pub, &msg)?;
            }
        }

        // Publish control status every tick
        {
            let mut msg = capnp::message::Builder::new_default();
            {
                let mut state = msg.init_root::<behave::schema::control_status_capnp::control_status::Builder<'_>>();
                state.set_armed(armed);
                state.set_mode(mode);
                state.set_battery_pct(100.0);
            }
            topics::control::status::send(&ctrl_pub, &msg)?;
        }
    }

    warn!("node loop exited");
    Ok(())
}
