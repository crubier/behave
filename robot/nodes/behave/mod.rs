//! Behave node -- tick-based behavior tree executor.
//!
//! Fully action-agnostic: receives missions, delegates everything
//! to `behave::actions::from_capnp()` and `behave::actions::tick()`.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::action::Tick;
use behave::controls::CmdPublisher;
use behave::ipc::{self, CmdMessage, MissionMessage};
use behave::schema::mission_capnp::mission;

struct Iox2CmdPublisher<'a> {
    inner: &'a iceoryx2::port::publisher::Publisher<
        iceoryx2::prelude::ipc::Service,
        CmdMessage,
        (),
    >,
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

    let mission_service = node
        .service_builder(&"behave/Mission".try_into()?)
        .publish_subscribe::<MissionMessage>()
        .open_or_create()?;
    let mission_sub = mission_service.subscriber_builder().create()?;
    info!("subscribed to behave/Mission");

    let cmd_service = node
        .service_builder(&"behave/ControlCommand".try_into()?)
        .publish_subscribe::<CmdMessage>()
        .open_or_create()?;
    let cmd_pub = cmd_service.publisher_builder().create()?;
    info!("publishing behave/ControlCommand");

    let publisher = Iox2CmdPublisher { inner: &cmd_pub };
    info!("ready -- waiting for missions");

    let mut active_mission: Option<(u64, behave::actions::ActionNode)> = None;

    while node.wait(Duration::from_millis(100)).is_ok() {
        // Accept new missions
        while let Some(sample) = mission_sub.receive()? {
            let typed = ipc::unpack::<{ ipc::MISSION_BUF }, mission::Owned>(&*sample)?;
            let m = typed.get()?;
            let mid = m.get_id();
            info!("=== mission #{mid} received ===");

            let root = behave::actions::from_capnp(&m.get_root()?)?;
            active_mission = Some((mid, root));
            info!("=== mission #{mid} ready ===");
        }

        // Tick active mission
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
