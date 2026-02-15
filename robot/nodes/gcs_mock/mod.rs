//! Mock GCS node -- sends a sample action tree over UDP after a short delay.

use anyhow::Result;
use log::info;
use prost::Message;
use tokio::net::UdpSocket;

use behave::actions::action_proto::{action_args, ActionArgs, ActionNode};
use behave::actions::takeoff::proto::TakeoffArgs;
use behave::actions::land::proto::LandArgs;
use behave::actions::goto_waypoint::proto::GotoWaypointArgs;
use behave::actions::return_home::proto::ReturnHomeArgs;
use behave::actions::take_photo::proto::TakePhotoArgs;
use behave::actions::sequence::proto::SequenceArgs;

const TARGET: &str = "127.0.0.1:9000";
const STARTUP_DELAY_SECS: u64 = 3;

pub fn run() -> Result<()> {
    behave::logging::init("MockGCS");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(async { run_async().await })
}

fn leaf(id: u64, action: action_args::Action) -> ActionNode {
    ActionNode {
        id,
        args: Some(ActionArgs { action: Some(action) }),
        children: vec![],
    }
}

async fn run_async() -> Result<()> {
    info!("waiting {STARTUP_DELAY_SECS}s for other nodes to start...");
    tokio::time::sleep(std::time::Duration::from_secs(STARTUP_DELAY_SECS)).await;

    info!("building sample action tree...");

    let root = ActionNode {
        id: 100,
        args: Some(ActionArgs {
            action: Some(action_args::Action::Sequence(SequenceArgs {})),
        }),
        children: vec![
            leaf(101, action_args::Action::Takeoff(TakeoffArgs { altitude_m: 50.0 })),
            leaf(102, action_args::Action::GotoWaypoint(GotoWaypointArgs {
                easting_m: 500.0, northing_m: 300.0, altitude_m: 80.0, speed_ms: 15.0,
            })),
            leaf(103, action_args::Action::TakePhoto(TakePhotoArgs {})),
            leaf(104, action_args::Action::GotoWaypoint(GotoWaypointArgs {
                easting_m: 1200.0, northing_m: -150.0, altitude_m: 80.0, speed_ms: 15.0,
            })),
            leaf(105, action_args::Action::TakePhoto(TakePhotoArgs {})),
            leaf(106, action_args::Action::ReturnHome(ReturnHomeArgs { altitude_m: 60.0 })),
            leaf(107, action_args::Action::Land(LandArgs { descent_speed_ms: 2.0 })),
        ],
    };

    let buf = root.encode_to_vec();
    info!("action tree serialized ({} bytes)", buf.len());
    info!("sending to {TARGET}...");

    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.send_to(&buf, TARGET).await?;

    info!("action tree sent!");
    Ok(())
}
