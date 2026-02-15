//! Behave node -- tick-based behavior tree executor.
//!
//! Fully action-agnostic: receives ActionArgs, delegates everything
//! to `behave::actions::from_capnp()` and `behave::actions::tick()`.

use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::actions::Tick;
use behave::controls::CmdPublisher;
use behave::ipc::CmdMessage;
use behave::schema::actions::action_capnp::action_args;
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

    let action_sub = topics::behave::request::subscribe(&node)?;
    info!("subscribed to {}", topics::behave::request::NAME);

    let cmd_pub = topics::control::request::publish(&node)?;
    info!("publishing {}", topics::control::request::NAME);

    let publisher = Iox2CmdPublisher { inner: &cmd_pub };
    info!("ready -- waiting for action requests");

    let mut active_action: Option<(u64, behave::actions::ActionNode)> = None;

    while node.wait(Duration::from_millis(100)).is_ok() {
        while let Some(typed) = topics::receive::<{ topics::behave::request::BUF }, action_args::Owned>(&action_sub)? {
            let args = typed.get()?;
            let id = args.get_id();
            let name = args.get_name()?.to_str().unwrap_or("?");
            info!("=== action #{id} \"{name}\" received ===");

            let root = behave::actions::from_capnp(&args)?;
            active_action = Some((id, root));
            info!("=== action #{id} ready ===");
        }

        if let Some((id, ref mut root)) = active_action {
            match behave::actions::tick(root, &publisher) {
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
