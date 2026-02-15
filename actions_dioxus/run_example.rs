//! Standalone runner for the example mission.
//!
//! Simulates a drone (position updates each tick) and runs the
//! ExampleMission through the Dioxus BehaviorTreeRenderer.
//!
//! Run with: `cargo run --bin run-example-mission`

use std::thread;
use std::time::Duration;

use anyhow::Result;
use dioxus_core::VirtualDom;
use log::info;

use behave::actions_dioxus::ActionIO;
use behave::actions_dioxus::core::data::RunStatus;
use behave::actions_dioxus::core::renderer::BehaviorTreeRenderer;
use behave::actions_dioxus::example_mission::ExampleMission;
use behave::controls::CmdPublisher;
use behave::topics::control::request::ControlRequest;
use behave::topics::control::response::ControlResponse;
use behave::topics::control::status::ControlStatus;
use behave::topics::sense::status::SenseStatus;

/// Mock command publisher -- just logs commands.
struct MockCmd;
impl CmdPublisher for MockCmd {
    fn send_cmd(&self, cmd: ControlRequest) -> Result<()> {
        info!("[mock-cmd] {:?}", cmd);
        Ok(())
    }
}

/// Simple drone simulator state.
struct DroneSim {
    easting_m: f64,
    northing_m: f64,
    altitude_m: f64,
    target_easting_m: f64,
    target_northing_m: f64,
    target_altitude_m: f64,
    speed_ms: f64,
    landing: bool,
    returning: bool,
}

impl DroneSim {
    fn new() -> Self {
        Self {
            easting_m: 0.0,
            northing_m: 0.0,
            altitude_m: 0.0,
            target_easting_m: 0.0,
            target_northing_m: 0.0,
            target_altitude_m: 0.0,
            speed_ms: 0.0,
            landing: false,
            returning: false,
        }
    }

    fn tick(&mut self, dt: f64) {
        if self.landing {
            self.altitude_m = (self.altitude_m - 2.0 * dt).max(0.0);
            return;
        }
        if self.returning {
            // Fly back to origin
            let de = -self.easting_m;
            let dn = -self.northing_m;
            let dist = (de * de + dn * dn).sqrt();
            if dist > 1.0 {
                self.easting_m += de / dist * 20.0 * dt;
                self.northing_m += dn / dist * 20.0 * dt;
            } else {
                self.returning = false;
            }
            return;
        }

        // Altitude
        let alt_diff = self.target_altitude_m - self.altitude_m;
        if alt_diff.abs() > 0.5 {
            self.altitude_m += alt_diff.signum() * 10.0 * dt;
        }

        // Horizontal movement
        let de = self.target_easting_m - self.easting_m;
        let dn = self.target_northing_m - self.northing_m;
        let dist = (de * de + dn * dn).sqrt();
        if dist > 1.0 && self.speed_ms > 0.0 {
            self.easting_m += de / dist * self.speed_ms * dt;
            self.northing_m += dn / dist * self.speed_ms * dt;
        }
    }

    fn handle_cmd(&mut self, cmd: &ControlRequest) {
        use behave::topics::control::request::*;
        match cmd.cmd {
            CMD_TAKEOFF => {
                self.target_altitude_m = cmd.altitude_m;
                self.landing = false;
                self.returning = false;
            }
            CMD_GOTO => {
                self.target_easting_m = cmd.easting_m;
                self.target_northing_m = cmd.northing_m;
                self.target_altitude_m = cmd.altitude_m;
                self.speed_ms = cmd.speed_ms;
            }
            CMD_RETURN_HOME => {
                self.returning = true;
                self.landing = false;
            }
            CMD_LAND => {
                self.landing = true;
                self.returning = false;
            }
            CMD_TRIGGER_CAMERA => {
                // instant ack
            }
            _ => {}
        }
    }

    fn sense(&self) -> SenseStatus {
        SenseStatus {
            easting_m: self.easting_m,
            northing_m: self.northing_m,
            altitude_m: self.altitude_m,
        }
    }

    fn control_status(&self) -> ControlStatus {
        ControlStatus {
            armed: true,
            mode: if self.returning { 5 } else { 4 }, // MODE_RETURNING = 5
            battery_pct: 100.0,
        }
    }
}

/// Mock publisher that records commands for the sim.
struct SimCmd {
    cmds: std::cell::RefCell<Vec<ControlRequest>>,
}

impl SimCmd {
    fn new() -> Self {
        Self { cmds: std::cell::RefCell::new(Vec::new()) }
    }
    fn drain(&self) -> Vec<ControlRequest> {
        self.cmds.borrow_mut().drain(..).collect()
    }
}

impl CmdPublisher for SimCmd {
    fn send_cmd(&self, cmd: ControlRequest) -> Result<()> {
        info!("[cmd] cmd={} alt={:.1} e={:.1} n={:.1} spd={:.1}",
            cmd.cmd, cmd.altitude_m, cmd.easting_m, cmd.northing_m, cmd.speed_ms);
        self.cmds.borrow_mut().push(cmd);
        Ok(())
    }
}

fn main() {
    behave::logging::init("ExampleMission");
    info!("=== starting example mission ===");

    let mut dom = VirtualDom::new(ExampleMission);
    let mut renderer = BehaviorTreeRenderer::new();

    // Initial render -- builds the element tree
    dom.rebuild(&mut renderer);
    info!("element tree built ({} nodes)", renderer.node_count());

    // Simulated drone
    let mut sim = DroneSim::new();
    let sim_cmd = SimCmd::new();

    // Activate the root
    let io = ActionIO {
        cmd: &sim_cmd,
        control_response: ControlResponse::default(),
        control_status: sim.control_status(),
        sense_status: sim.sense(),
    };
    renderer.activate_root(&io);

    // Process any initial commands
    for cmd in sim_cmd.drain() {
        sim.handle_cmd(&cmd);
    }

    // Tick loop
    let dt = 0.1; // 100ms
    let mut tick = 0u64;
    loop {
        tick += 1;
        sim.tick(dt);

        let io = ActionIO {
            cmd: &sim_cmd,
            control_response: ControlResponse::new(0, true, "ok"), // always ack
            control_status: sim.control_status(),
            sense_status: sim.sense(),
        };

        renderer.tick_actions(&io);
        dom.render_immediate(&mut renderer);

        // Process commands from this tick
        for cmd in sim_cmd.drain() {
            sim.handle_cmd(&cmd);
        }

        // Check if done
        if let Some(run) = renderer.take_mission_run() {
            info!("=== mission {:?} (run #{}, {} ticks, {:.1}s) ===",
                run.status, run.id, tick, tick as f64 * dt);
            break;
        }

        // Status every 10 ticks
        if tick % 10 == 0 {
            info!("[sim] tick={} pos=({:.1}, {:.1}) alt={:.1}m",
                tick, sim.easting_m, sim.northing_m, sim.altitude_m);
        }

        thread::sleep(Duration::from_millis(10)); // 10x speed
    }
}
