//! Control node -- drone flight controller interface.
//!
//! Subscribes to `behave/ControlCommand` and executes each command
//! (stubbed: just logs). Publishes `behave/ControlAck` and
//! `behave/DroneState` telemetry.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::ipc::{self, AckMessage, CmdMessage, StateMessage};
use behave::schema::controls_capnp::{control_command, FlightMode};

pub fn run() -> Result<()> {
    behave::logging::init("Control");

    info!("starting");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let cmd_service = node
        .service_builder(&"behave/ControlCommand".try_into()?)
        .publish_subscribe::<CmdMessage>()
        .open_or_create()?;
    let cmd_sub = cmd_service.subscriber_builder().create()?;
    info!("subscribed to behave/ControlCommand");

    let ack_service = node
        .service_builder(&"behave/ControlAck".try_into()?)
        .publish_subscribe::<AckMessage>()
        .open_or_create()?;
    let ack_pub = ack_service.publisher_builder().create()?;
    info!("publishing behave/ControlAck");

    let state_service = node
        .service_builder(&"behave/DroneState".try_into()?)
        .publish_subscribe::<StateMessage>()
        .open_or_create()?;
    let state_pub = state_service.publisher_builder().create()?;
    info!("publishing behave/DroneState");

    // Simulated drone state
    let mut lat = 48.8566_f64;
    let mut lon = 2.3522_f64;
    let mut alt = 0.0_f64;
    let mut armed = false;
    let mut mode = FlightMode::Idle;

    info!("ready -- waiting for commands");

    while node.wait(Duration::from_millis(100)).is_ok() {
        while let Some(sample) = cmd_sub.receive()? {
            let typed = ipc::unpack::<{ ipc::CMD_BUF }, control_command::Owned>(&*sample)?;
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
                    info!(
                        "cmd #{cmd_id}: GOTO ({t_lat:.6}, {t_lon:.6}) alt={t_alt:.1}m spd={t_spd:.1}m/s"
                    );
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
                let mut capnp_msg = capnp::message::Builder::new_default();
                {
                    let mut ack = capnp_msg
                        .init_root::<behave::schema::controls_capnp::control_ack::Builder<'_>>();
                    ack.set_command_id(cmd_id);
                    ack.set_success(true);
                    ack.set_message("ok".into());
                }
                let envelope = ipc::pack::<{ ipc::ACK_BUF }>(&capnp_msg)?;
                let sample = ack_pub.loan_uninit()?;
                sample.write_payload(envelope).send()?;
            }
        }

        // Publish telemetry (suppress logging for this high-frequency event)
        {
            let mut capnp_msg = capnp::message::Builder::new_default();
            {
                let mut state = capnp_msg
                    .init_root::<behave::schema::controls_capnp::drone_state::Builder<'_>>();
                state.set_latitude_deg(lat);
                state.set_longitude_deg(lon);
                state.set_altitude_m(alt);
                state.set_armed(armed);
                state.set_mode(mode);
                state.set_battery_pct(95.0);
            }
            let envelope = ipc::pack::<{ ipc::STATE_BUF }>(&capnp_msg)?;
            let sample = state_pub.loan_uninit()?;
            sample.write_payload(envelope).send()?;
        }
    }

    warn!("node loop exited");
    Ok(())
}
