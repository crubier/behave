//! Behave node -- behavior tree executor.
//!
//! Subscribes to `behave/Mission` (from Communicate), walks the ActionSpec
//! tree, and publishes `behave/ControlCommand` messages to the Control node.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::ipc::{self, CmdMessage, MissionMessage};
use behave::schema::actions::action_capnp::action_spec;
use behave::schema::mission_capnp::mission;

static CMD_COUNTER: AtomicU64 = AtomicU64::new(1);

trait CmdPublisher {
    fn send_envelope(&self, envelope: CmdMessage) -> Result<()>;
}

struct Iox2CmdPublisher<'a> {
    inner: &'a iceoryx2::port::publisher::Publisher<
        iceoryx2::prelude::ipc::Service,
        CmdMessage,
        ()
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

    while node.wait(Duration::from_millis(200)).is_ok() {
        while let Some(sample) = mission_sub.receive()? {
            let typed =
                ipc::unpack::<{ ipc::MISSION_BUF }, mission::Owned>(&*sample)?;
            let m = typed.get()?;
            let mid = m.get_id();
            info!("=== mission #{mid} received ===");

            let root = m.get_root()?;
            execute_node(&publisher, &root, 0)?;

            info!("=== mission #{mid} complete ===");
        }
    }

    warn!("node loop exited");
    Ok(())
}

fn execute_node(cmd_pub: &dyn CmdPublisher, spec: &action_spec::Reader<'_>, depth: usize) -> Result<()> {
    let indent = "  ".repeat(depth);
    let name = spec.get_name()?.to_str().unwrap_or("(unnamed)");
    let id = spec.get_id();

    match spec.which()? {
        action_spec::Sequence(_) => {
            let children = spec.get_children()?;
            info!("{indent}[#{id}] SEQUENCE \"{name}\" ({} children)", children.len());
            for i in 0..children.len() {
                execute_node(cmd_pub, &children.get(i), depth + 1)?;
            }
            info!("{indent}[#{id}] SEQUENCE \"{name}\" done");
        }
        action_spec::Fallback(_) => {
            let children = spec.get_children()?;
            info!("{indent}[#{id}] FALLBACK \"{name}\" ({} children)", children.len());
            for i in 0..children.len() {
                execute_node(cmd_pub, &children.get(i), depth + 1)?;
            }
            info!("{indent}[#{id}] FALLBACK \"{name}\" done");
        }
        action_spec::Takeoff(r) => {
            let r = r?;
            let alt = r.get_altitude_m();
            info!("{indent}[#{id}] TAKEOFF alt={alt:.1}m -> Control");
            send_takeoff(cmd_pub, alt)?;
        }
        action_spec::GotoWaypoint(r) => {
            let r = r?;
            let lat = r.get_latitude_deg();
            let lon = r.get_longitude_deg();
            let alt = r.get_altitude_m();
            let spd = r.get_speed_ms();
            info!("{indent}[#{id}] GOTO ({lat:.6}, {lon:.6}) alt={alt:.1}m spd={spd:.1}m/s -> Control");
            send_goto(cmd_pub, lat, lon, alt, spd)?;
        }
        action_spec::ReturnHome(r) => {
            let r = r?;
            let alt = r.get_altitude_m();
            info!("{indent}[#{id}] RETURN HOME alt={alt:.1}m -> Control");
            send_return_home(cmd_pub, alt)?;
        }
        action_spec::Land(r) => {
            let r = r?;
            let spd = r.get_descent_speed_ms();
            info!("{indent}[#{id}] LAND descent={spd:.1}m/s -> Control");
            send_land(cmd_pub, spd)?;
        }
        action_spec::TakePhoto(r) => {
            let r = r?;
            let tag = r.get_tag()?.to_str()?;
            info!("{indent}[#{id}] TAKE PHOTO tag=\"{tag}\" -> Control");
            send_camera(cmd_pub, tag)?;
        }
    }

    Ok(())
}

fn next_cmd_id() -> u64 {
    CMD_COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn send_takeoff(cmd_pub: &dyn CmdPublisher, alt: f64) -> Result<()> {
    let mut capnp_msg = capnp::message::Builder::new_default();
    {
        let mut cmd = capnp_msg
            .init_root::<behave::schema::controls_capnp::control_command::Builder<'_>>();
        cmd.set_id(next_cmd_id());
        cmd.init_takeoff().set_altitude_m(alt);
    }
    cmd_pub.send_envelope(ipc::pack::<{ ipc::CMD_BUF }>(&capnp_msg)?)
}

fn send_goto(cmd_pub: &dyn CmdPublisher, lat: f64, lon: f64, alt: f64, spd: f64) -> Result<()> {
    let mut capnp_msg = capnp::message::Builder::new_default();
    {
        let mut cmd = capnp_msg
            .init_root::<behave::schema::controls_capnp::control_command::Builder<'_>>();
        cmd.set_id(next_cmd_id());
        let mut g = cmd.init_goto();
        g.set_latitude_deg(lat);
        g.set_longitude_deg(lon);
        g.set_altitude_m(alt);
        g.set_speed_ms(spd);
    }
    cmd_pub.send_envelope(ipc::pack::<{ ipc::CMD_BUF }>(&capnp_msg)?)
}

fn send_return_home(cmd_pub: &dyn CmdPublisher, alt: f64) -> Result<()> {
    let mut capnp_msg = capnp::message::Builder::new_default();
    {
        let mut cmd = capnp_msg
            .init_root::<behave::schema::controls_capnp::control_command::Builder<'_>>();
        cmd.set_id(next_cmd_id());
        cmd.init_return_home().set_altitude_m(alt);
    }
    cmd_pub.send_envelope(ipc::pack::<{ ipc::CMD_BUF }>(&capnp_msg)?)
}

fn send_land(cmd_pub: &dyn CmdPublisher, spd: f64) -> Result<()> {
    let mut capnp_msg = capnp::message::Builder::new_default();
    {
        let mut cmd = capnp_msg
            .init_root::<behave::schema::controls_capnp::control_command::Builder<'_>>();
        cmd.set_id(next_cmd_id());
        cmd.init_land().set_descent_speed_ms(spd);
    }
    cmd_pub.send_envelope(ipc::pack::<{ ipc::CMD_BUF }>(&capnp_msg)?)
}

fn send_camera(cmd_pub: &dyn CmdPublisher, tag: &str) -> Result<()> {
    let mut capnp_msg = capnp::message::Builder::new_default();
    {
        let mut cmd = capnp_msg
            .init_root::<behave::schema::controls_capnp::control_command::Builder<'_>>();
        cmd.set_id(next_cmd_id());
        cmd.init_trigger_camera().set_tag(tag.into());
    }
    cmd_pub.send_envelope(ipc::pack::<{ ipc::CMD_BUF }>(&capnp_msg)?)
}
