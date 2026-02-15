//! Control Mock node -- dumb stub that just logs and acks every command.
//!
//! No state tracking, no simulation. Just receives ControlCommands,
//! logs them, and immediately acks with success.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::schema::controls_capnp::control_command;
use behave::topics;

pub fn run() -> Result<()> {
    behave::logging::init("CtrlMock");
    info!("starting (dumb mock -- logs and acks everything)");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let cmd_sub = topics::control::command::subscribe(&node)?;
    let ack_pub = topics::control::ack::publish(&node)?;

    info!("ready");

    while node.wait(Duration::from_millis(100)).is_ok() {
        while let Some(typed) = topics::receive::<{ topics::control::command::BUF }, control_command::Owned>(&cmd_sub)? {
            let cmd = typed.get()?;
            let cmd_id = cmd.get_id();

            let label = match cmd.which()? {
                control_command::Arm(()) => "ARM".to_string(),
                control_command::Disarm(()) => "DISARM".to_string(),
                control_command::Takeoff(r) => format!("TAKEOFF {:.1}m", r?.get_altitude_m()),
                control_command::Land(r) => format!("LAND {:.1}m/s", r?.get_descent_speed_ms()),
                control_command::Hover(()) => "HOVER".to_string(),
                control_command::ReturnHome(r) => format!("RTH {:.1}m", r?.get_altitude_m()),
                control_command::Goto(r) => {
                    let r = r?;
                    format!("GOTO ({:.4},{:.4})", r.get_latitude_deg(), r.get_longitude_deg())
                }
                control_command::TriggerCamera(r) => format!("CAMERA \"{}\"", r?.get_tag()?.to_str()?),
            };

            info!("cmd #{cmd_id}: {label} -> ACK ok");

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

    warn!("node loop exited");
    Ok(())
}
