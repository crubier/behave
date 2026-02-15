//! Control node -- drone flight controller interface.
//!
//! Subscribes to ControlCommand and executes each command (stubbed).
//! Publishes ControlAck and DroneState telemetry.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::schema::controls_capnp::{control_command, FlightMode};
use behave::topics;

pub fn run() -> Result<()> {
    behave::logging::init("Control");
    info!("starting");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let cmd_sub = topics::control::command::subscribe(&node)?;
    info!("subscribed to {}", topics::control::command::NAME);

    let ack_pub = topics::control::ack::publish(&node)?;
    info!("publishing {}", topics::control::ack::NAME);

    let state_pub = topics::control::state::publish(&node)?;
    info!("publishing {}", topics::control::state::NAME);

    let mut lat = 48.8566_f64;
    let mut lon = 2.3522_f64;
    let mut alt = 0.0_f64;
    let mut armed = false;
    let mut mode = FlightMode::Idle;

    info!("ready -- waiting for commands");

    while node.wait(Duration::from_millis(100)).is_ok() {
        while let Some(typed) = topics::receive::<{ topics::control::command::BUF }, control_command::Owned>(&cmd_sub)? {
            let cmd = typed.get()?;
            let cmd_id = cmd.get_id();

            match cmd.which()? {
                control_command::Arm(()) => {
                    info!("cmd #{cmd_id}: ARM");
                    armed = true;
                    mode = FlightMode::Idle;
                }
                control_command::Disarm(()) => {
                    info!("cmd #{cmd_id}: DISARM");
                    armed = false;
                    mode = FlightMode::Idle;
                }
                control_command::Takeoff(r) => {
                    let r = r?;
                    let target_alt = r.get_altitude_m();
                    info!("cmd #{cmd_id}: TAKEOFF to {target_alt:.1} m");
                    alt = target_alt;
                    mode = FlightMode::TakingOff;
                }
                control_command::Land(r) => {
                    let r = r?;
                    let spd = r.get_descent_speed_ms();
                    info!("cmd #{cmd_id}: LAND at {spd:.1} m/s descent");
                    alt = 0.0;
                    mode = FlightMode::Landing;
                }
                control_command::Hover(()) => {
                    info!("cmd #{cmd_id}: HOVER");
                    mode = FlightMode::Hovering;
                }
                control_command::ReturnHome(r) => {
                    let r = r?;
                    let rth_alt = r.get_altitude_m();
                    info!("cmd #{cmd_id}: RETURN HOME at {rth_alt:.1} m");
                    alt = rth_alt;
                    mode = FlightMode::Returning;
                }
                control_command::Goto(r) => {
                    let r = r?;
                    let t_lat = r.get_latitude_deg();
                    let t_lon = r.get_longitude_deg();
                    let t_alt = r.get_altitude_m();
                    let t_spd = r.get_speed_ms();
                    info!("cmd #{cmd_id}: GOTO ({t_lat:.6}, {t_lon:.6}) alt={t_alt:.1}m spd={t_spd:.1}m/s");
                    lat = t_lat;
                    lon = t_lon;
                    alt = t_alt;
                    mode = FlightMode::Flying;
                }
                control_command::TriggerCamera(r) => {
                    let r = r?;
                    let tag = r.get_tag()?.to_str()?;
                    info!("cmd #{cmd_id}: CAMERA TRIGGER tag=\"{tag}\"");
                }
            }

            info!("cmd #{cmd_id}: ACK ok");

            // Send ack
            {
                let mut msg = capnp::message::Builder::new_default();
                {
                    let mut ack = msg.init_root::<behave::schema::controls_capnp::control_ack::Builder<'_>>();
                    ack.set_command_id(cmd_id);
                    ack.set_success(true);
                    ack.set_message("ok".into());
                }
                topics::control::ack::send(&ack_pub, &msg)?;
            }
        }

        // Publish telemetry
        {
            let mut msg = capnp::message::Builder::new_default();
            {
                let mut state = msg.init_root::<behave::schema::controls_capnp::drone_state::Builder<'_>>();
                state.set_latitude_deg(lat);
                state.set_longitude_deg(lon);
                state.set_altitude_m(alt);
                state.set_armed(armed);
                state.set_mode(mode);
                state.set_battery_pct(95.0);
            }
            topics::control::state::send(&state_pub, &msg)?;
        }
    }

    warn!("node loop exited");
    Ok(())
}
