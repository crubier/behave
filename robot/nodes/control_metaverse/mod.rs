//! Control Metaverse node -- drone flight controller for the UE5 metaverse sim.
//!
//! Subscribes to ControlRequest and executes each command (stubbed).
//! Publishes ControlResponse and ControlStatus telemetry.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::schema::control_request_capnp::control_request;
use behave::schema::control_status_capnp::FlightMode;
use behave::topics;

pub fn run() -> Result<()> {
    behave::logging::init("CtrlMeta");
    info!("starting");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let cmd_sub = topics::control::request::subscribe(&node)?;
    info!("subscribed to {}", topics::control::request::NAME);

    let ack_pub = topics::control::response::publish(&node)?;
    info!("publishing {}", topics::control::response::NAME);

    let state_pub = topics::control::status::publish(&node)?;
    info!("publishing {}", topics::control::status::NAME);

    let mut armed = false;
    let mut mode = FlightMode::Idle;

    info!("ready -- waiting for commands");

    while node.wait(Duration::from_millis(100)).is_ok() {
        while let Some(typed) = topics::receive::<{ topics::control::request::BUF }, control_request::Owned>(&cmd_sub)? {
            let cmd = typed.get()?;
            let cmd_id = cmd.get_id();

            match cmd.which()? {
                control_request::Arm(()) => {
                    info!("cmd #{cmd_id}: ARM");
                    armed = true;
                    mode = FlightMode::Idle;
                }
                control_request::Disarm(()) => {
                    info!("cmd #{cmd_id}: DISARM");
                    armed = false;
                    mode = FlightMode::Idle;
                }
                control_request::Takeoff(r) => {
                    let target_alt = r?.get_altitude_m();
                    info!("cmd #{cmd_id}: TAKEOFF to {target_alt:.1} m");
                    mode = FlightMode::TakingOff;
                }
                control_request::Land(r) => {
                    let spd = r?.get_descent_speed_ms();
                    info!("cmd #{cmd_id}: LAND at {spd:.1} m/s descent");
                    mode = FlightMode::Landing;
                }
                control_request::Hover(()) => {
                    info!("cmd #{cmd_id}: HOVER");
                    mode = FlightMode::Hovering;
                }
                control_request::ReturnHome(r) => {
                    let rth_alt = r?.get_altitude_m();
                    info!("cmd #{cmd_id}: RETURN HOME at {rth_alt:.1} m");
                    mode = FlightMode::Returning;
                }
                control_request::Goto(r) => {
                    let r = r?;
                    info!("cmd #{cmd_id}: GOTO ({:.1}, {:.1}) alt={:.1}m spd={:.1}m/s",
                        r.get_easting_m(), r.get_northing_m(), r.get_altitude_m(), r.get_speed_ms());
                    mode = FlightMode::Flying;
                }
                control_request::TriggerCamera(r) => {
                    let tag = r?.get_tag()?.to_str()?;
                    info!("cmd #{cmd_id}: CAMERA TRIGGER tag=\"{tag}\"");
                }
            }

            info!("cmd #{cmd_id}: ACK ok");

            // Send ack
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
        }

        // Publish telemetry
        {
            let mut msg = capnp::message::Builder::new_default();
            {
                let mut state = msg.init_root::<behave::schema::control_status_capnp::control_status::Builder<'_>>();
                state.set_armed(armed);
                state.set_mode(mode);
                state.set_battery_pct(95.0);
            }
            topics::control::status::send(&state_pub, &msg)?;
        }
    }

    warn!("node loop exited");
    Ok(())
}
