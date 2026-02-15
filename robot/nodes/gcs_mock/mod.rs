//! Mock GCS node -- sends a sample action tree over UDP after a short delay.
//!
//! Simulates a ground control station sending a "Survey Mission" behavior
//! tree (as FlatBuffer ActionArgs) to the Communicate node.

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

    // Build children bottom-up (FlatBuffers requires this)
    let takeoff_args = TakeoffArgs::create(&mut builder, &TakeoffArgsArgs { altitude_m: 50.0 });
    let takeoff_name = builder.create_string("takeoff");
    let takeoff = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 101, name: Some(takeoff_name),
        action_type: ActionArgsType::TakeoffArgs, action: Some(takeoff_args.as_union_value()),
        children: None,
    });

    let goto1_args = GotoWaypointArgs::create(&mut builder, &GotoWaypointArgsArgs {
        easting_m: 500.0, northing_m: 300.0, altitude_m: 80.0, speed_ms: 15.0,
    });
    let goto1_name = builder.create_string("goto_eiffel");
    let goto1 = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 102, name: Some(goto1_name),
        action_type: ActionArgsType::GotoWaypointArgs, action: Some(goto1_args.as_union_value()),
        children: None,
    });

    let photo1_tag = builder.create_string("eiffel_tower");
    let photo1_args = TakePhotoArgs::create(&mut builder, &TakePhotoArgsArgs { tag: Some(photo1_tag) });
    let photo1_name = builder.create_string("photo_eiffel");
    let photo1 = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 103, name: Some(photo1_name),
        action_type: ActionArgsType::TakePhotoArgs, action: Some(photo1_args.as_union_value()),
        children: None,
    });

    let goto2_args = GotoWaypointArgs::create(&mut builder, &GotoWaypointArgsArgs {
        easting_m: 1200.0, northing_m: -150.0, altitude_m: 80.0, speed_ms: 15.0,
    });
    let goto2_name = builder.create_string("goto_notre_dame");
    let goto2 = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 104, name: Some(goto2_name),
        action_type: ActionArgsType::GotoWaypointArgs, action: Some(goto2_args.as_union_value()),
        children: None,
    });

    let photo2_tag = builder.create_string("notre_dame");
    let photo2_args = TakePhotoArgs::create(&mut builder, &TakePhotoArgsArgs { tag: Some(photo2_tag) });
    let photo2_name = builder.create_string("photo_notre_dame");
    let photo2 = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 105, name: Some(photo2_name),
        action_type: ActionArgsType::TakePhotoArgs, action: Some(photo2_args.as_union_value()),
        children: None,
    });

    let rth_args = ReturnHomeArgs::create(&mut builder, &ReturnHomeArgsArgs { altitude_m: 60.0 });
    let rth_name = builder.create_string("return_home");
    let rth = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 106, name: Some(rth_name),
        action_type: ActionArgsType::ReturnHomeArgs, action: Some(rth_args.as_union_value()),
        children: None,
    });

    let land_args = LandArgs::create(&mut builder, &LandArgsArgs { descent_speed_ms: 2.0 });
    let land_name = builder.create_string("land");
    let land = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 107, name: Some(land_name),
        action_type: ActionArgsType::LandArgs, action: Some(land_args.as_union_value()),
        children: None,
    });

    // Build sequence root with children
    let children = builder.create_vector(&[takeoff, goto1, photo1, goto2, photo2, rth, land]);
    let seq_args = SequenceArgs::create(&mut builder, &Default::default());
    let root_name = builder.create_string("Survey Mission");
    let root = FbActionArgs::create(&mut builder, &FbActionArgsArgs {
        id: 100, name: Some(root_name),
        action_type: ActionArgsType::SequenceArgs, action: Some(seq_args.as_union_value()),
        children: Some(children),
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
