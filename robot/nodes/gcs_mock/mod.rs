//! Mock GCS node -- sends a sample action tree over UDP after a short delay.

use anyhow::Result;
use log::info;
use tokio::net::UdpSocket;

use behave::schema::behave::actions::{
    ActionArgs as FbActionArgs, ActionArgsArgs as FbActionArgsArgs,
    ActionArgsType,
    TakeoffArgs, TakeoffArgsArgs,
    LandArgs, LandArgsArgs,
    GotoWaypointArgs, GotoWaypointArgsArgs,
    ReturnHomeArgs, ReturnHomeArgsArgs,
    TakePhotoArgs, TakePhotoArgsArgs,
    SequenceArgs,
};

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

    info!("building sample action tree...");

    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(2048);

    let takeoff_args = TakeoffArgs::create(&mut builder, &TakeoffArgsArgs { altitude_m: 50.0 });
    let takeoff = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 101, action_type: ActionArgsType::TakeoffArgs, action: Some(takeoff_args.as_union_value()), children: None,
    });

    let goto1_args = GotoWaypointArgs::create(&mut builder, &GotoWaypointArgsArgs {
        easting_m: 500.0, northing_m: 300.0, altitude_m: 80.0, speed_ms: 15.0,
    });
    let goto1 = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 102, action_type: ActionArgsType::GotoWaypointArgs, action: Some(goto1_args.as_union_value()), children: None,
    });

    let photo1_args = TakePhotoArgs::create(&mut builder, &TakePhotoArgsArgs {});
    let photo1 = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 103, action_type: ActionArgsType::TakePhotoArgs, action: Some(photo1_args.as_union_value()), children: None,
    });

    let goto2_args = GotoWaypointArgs::create(&mut builder, &GotoWaypointArgsArgs {
        easting_m: 1200.0, northing_m: -150.0, altitude_m: 80.0, speed_ms: 15.0,
    });
    let goto2 = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 104, action_type: ActionArgsType::GotoWaypointArgs, action: Some(goto2_args.as_union_value()), children: None,
    });

    let photo2_args = TakePhotoArgs::create(&mut builder, &TakePhotoArgsArgs {});
    let photo2 = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 105, action_type: ActionArgsType::TakePhotoArgs, action: Some(photo2_args.as_union_value()), children: None,
    });

    let rth_args = ReturnHomeArgs::create(&mut builder, &ReturnHomeArgsArgs { altitude_m: 60.0 });
    let rth = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 106, action_type: ActionArgsType::ReturnHomeArgs, action: Some(rth_args.as_union_value()), children: None,
    });

    let land_args = LandArgs::create(&mut builder, &LandArgsArgs { descent_speed_ms: 2.0 });
    let land = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 107, action_type: ActionArgsType::LandArgs, action: Some(land_args.as_union_value()), children: None,
    });

    let children = builder.create_vector(&[takeoff, goto1, photo1, goto2, photo2, rth, land]);
    let seq_args = SequenceArgs::create(&mut builder, &Default::default());
    let root = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 100, action_type: ActionArgsType::SequenceArgs, action: Some(seq_args.as_union_value()), children: Some(children),
    });

    builder.finish(root, None);
    let buf = builder.finished_data();

    info!("action tree serialized ({} bytes)", buf.len());
    info!("sending to {TARGET}...");

    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.send_to(buf, TARGET).await?;

    info!("action tree sent!");
    Ok(())
}
