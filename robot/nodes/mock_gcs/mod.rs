//! Mock GCS node -- sends a sample mission over UDP after a short delay.
//!
//! Simulates a ground control station sending a "Survey Mission" behavior
//! tree to the Communicate node.

use anyhow::Result;
use log::info;
use tokio::net::UdpSocket;

use behave::schema::mission_capnp::mission;

const TARGET: &str = "127.0.0.1:9000";
const STARTUP_DELAY_SECS: u64 = 3;

pub fn run() -> Result<()> {
    behave::logging::init("MockGCS");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(async { run_async().await })
}

async fn run_async() -> Result<()> {
    info!("waiting {STARTUP_DELAY_SECS}s for other nodes to start...");
    tokio::time::sleep(std::time::Duration::from_secs(STARTUP_DELAY_SECS)).await;

    info!("building sample mission...");

    let mut msg = capnp::message::Builder::new_default();
    {
        let mut m = msg.init_root::<mission::Builder<'_>>();
        m.set_id(1);

        let mut root = m.init_root();
        root.set_id(100);
        root.set_name("Survey Mission".into());

        root.reborrow().init_sequence();
        let mut children = root.init_children(7);

        {
            let mut node = children.reborrow().get(0);
            node.set_id(101);
            node.set_name("takeoff".into());
            node.init_takeoff().set_altitude_m(50.0);
        }
        {
            let mut node = children.reborrow().get(1);
            node.set_id(102);
            node.set_name("goto_eiffel".into());
            let mut g = node.init_goto_waypoint();
            g.set_latitude_deg(48.8584);
            g.set_longitude_deg(2.2945);
            g.set_altitude_m(80.0);
            g.set_speed_ms(15.0);
        }
        {
            let mut node = children.reborrow().get(2);
            node.set_id(103);
            node.set_name("photo_eiffel".into());
            node.init_take_photo().set_tag("eiffel_tower".into());
        }
        {
            let mut node = children.reborrow().get(3);
            node.set_id(104);
            node.set_name("goto_notre_dame".into());
            let mut g = node.init_goto_waypoint();
            g.set_latitude_deg(48.8606);
            g.set_longitude_deg(2.3376);
            g.set_altitude_m(80.0);
            g.set_speed_ms(15.0);
        }
        {
            let mut node = children.reborrow().get(4);
            node.set_id(105);
            node.set_name("photo_notre_dame".into());
            node.init_take_photo().set_tag("notre_dame".into());
        }
        {
            let mut node = children.reborrow().get(5);
            node.set_id(106);
            node.set_name("return_home".into());
            node.init_return_home().set_altitude_m(60.0);
        }
        {
            let mut node = children.reborrow().get(6);
            node.set_id(107);
            node.set_name("land".into());
            node.init_land().set_descent_speed_ms(2.0);
        }
    }

    let mut buf = Vec::new();
    capnp::serialize::write_message(&mut buf, &msg)?;

    info!("mission serialized ({} bytes)", buf.len());
    info!("sending to {TARGET}...");

    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.send_to(&buf, TARGET).await?;

    info!("mission sent!");
    Ok(())
}
