//! Behave-Dioxus node -- executes behavior trees using the Dioxus renderer.
//!
//! Replaces the original behave node. Runs ExampleMission directly
//! (no GCS/FlatBuffer needed). Connects to the same iceoryx2 topics
//! for control and sensing.

use std::time::Duration;

use anyhow::Result;
use dioxus_core::VirtualDom;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::actions_dioxus::ActionIO;
use behave::actions_dioxus::core::renderer::BehaviorTreeRenderer;
use behave::actions_dioxus::example_mission::ExampleMission;
use behave::controls::CmdPublisher;
use behave::topics;
use behave::topics::control::request::ControlRequest;
use behave::topics::control::response::ControlResponse;
use behave::topics::control::status::ControlStatus;
use behave::topics::sense::status::SenseStatus;

struct Iox2CmdPublisher<'a> {
    inner: &'a topics::NativePub<ControlRequest>,
}

impl<'a> CmdPublisher for Iox2CmdPublisher<'a> {
    fn send_cmd(&self, cmd: ControlRequest) -> Result<()> {
        topics::publish(self.inner, cmd)
    }
}

pub fn run() -> Result<()> {
    behave::logging::init("BehaveDioxus");
    info!("starting");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let cmd_pub = topics::control::request::publish(&node)?;
    info!("publishing {}", topics::control::request::NAME);

    let ctrl_resp_sub = topics::control::response::subscribe(&node)?;
    info!("subscribed to {}", topics::control::response::NAME);

    let ctrl_status_sub = topics::control::status::subscribe(&node)?;
    info!("subscribed to {}", topics::control::status::NAME);

    let sense_sub = topics::sense::status::subscribe(&node)?;
    info!("subscribed to {}", topics::sense::status::NAME);

    let publisher = Iox2CmdPublisher { inner: &cmd_pub };

    let mut snap_resp = ControlResponse::default();
    let mut snap_ctrl = ControlStatus::default();
    let mut snap_sense = SenseStatus::default();

    // ── Build the Dioxus element tree ───────────────────────
    let mut dom = VirtualDom::new(ExampleMission);
    let mut renderer = BehaviorTreeRenderer::new();
    dom.rebuild(&mut renderer);
    info!("element tree built ({} nodes)", renderer.node_count());

    // ── Activate the root ───────────────────────────────────
    let io = ActionIO {
        cmd: &publisher,
        control_response: snap_resp,
        control_status: snap_ctrl,
        sense_status: snap_sense,
    };
    renderer.activate_root(&io);
    info!("ready -- mission running");

    // ── Tick loop ───────────────────────────────────────────
    let mut tick = 0u64;
    while node.wait(Duration::from_millis(100)).is_ok() {
        tick += 1;

        // Poll native topics
        while let Some(r) = topics::receive_native(&ctrl_resp_sub)? { snap_resp = r; }
        while let Some(s) = topics::receive_native(&ctrl_status_sub)? { snap_ctrl = s; }
        while let Some(s) = topics::receive_native(&sense_sub)? { snap_sense = s; }

        let io = ActionIO {
            cmd: &publisher,
            control_response: snap_resp,
            control_status: snap_ctrl,
            sense_status: snap_sense,
        };

        // Tick all active nodes
        renderer.tick_actions(&io);

        // Let Dioxus reconcile any tree changes
        dom.render_immediate(&mut renderer);

        // Check if done
        if let Some(run) = renderer.take_mission_run() {
            info!("=== mission {:?} (run #{}, {} ticks) ===", run.status, run.id, tick);
            break;
        }

        if tick % 10 == 0 {
            info!("[tick {}] pos=({:.1}, {:.1}) alt={:.1}m",
                tick, snap_sense.easting_m, snap_sense.northing_m, snap_sense.altitude_m);
        }
    }

    warn!("node loop exited");
    Ok(())
}
