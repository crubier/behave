//! Behave node -- tick-based behavior tree executor.
//!
//! Fully action-agnostic: receives missions, delegates everything
//! to `behave::actions::from_capnp()` and `behave::actions::tick()`.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::actions::Tick;
use behave::controls::CmdPublisher;
use behave::ipc::CmdMessage;
use behave::schema::mission_capnp::mission;
use behave::topics;

struct Iox2CmdPublisher<'a> {
    inner: &'a topics::Pub<{ topics::control::command::BUF }>,
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

    let mission_sub = topics::mission::subscribe(&node)?;
    info!("subscribed to {}", topics::mission::NAME);

    let cmd_pub = topics::control::command::publish(&node)?;
    info!("publishing {}", topics::control::command::NAME);

    let publisher = Iox2CmdPublisher { inner: &cmd_pub };
    info!("ready -- waiting for missions");

    let mut active_mission: Option<(u64, behave::actions::ActionNode)> = None;

    while node.wait(Duration::from_millis(100)).is_ok() {
        while let Some(typed) = topics::receive::<{ topics::mission::BUF }, mission::Owned>(&mission_sub)? {
            let m = typed.get()?;
            let mid = m.get_id();
            info!("=== mission #{mid} received ===");

            let root = behave::actions::from_capnp(&m.get_root()?)?;
            active_mission = Some((mid, root));
            info!("=== mission #{mid} ready ===");
        }

        if let Some((mid, ref mut root)) = active_mission {
            match behave::actions::tick(root, &publisher) {
                Tick::Running(()) => {}
                Tick::Success(_) => {
                    info!("=== mission #{mid} SUCCESS ===");
                    active_mission = None;
                }
                Tick::Failure(_) => {
                    warn!("=== mission #{mid} FAILURE ===");
                    active_mission = None;
                }
            }
        }
    }

    warn!("node loop exited");
    Ok(())
}
