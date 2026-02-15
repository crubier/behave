fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // ── Topic schemas ───────────────────────────────────────────
    // Each topic folder has its own capnp schema. Compiled with
    // separate src_prefix per folder so module names stay flat
    // under `schema::`.

    capnpc::CompilerCommand::new()
        .src_prefix("robot/topics/control")
        .default_parent_module(vec!["schema".into()])
        .file("robot/topics/control/controls.capnp")
        .run()
        .expect("failed to compile controls schema");

    capnpc::CompilerCommand::new()
        .src_prefix("robot/topics/mission")
        .default_parent_module(vec!["schema".into()])
        .import_path(&root)
        .file("robot/topics/mission/mission.capnp")
        .run()
        .expect("failed to compile mission schema");

    capnpc::CompilerCommand::new()
        .src_prefix("robot/topics/sim_pose")
        .default_parent_module(vec!["schema".into()])
        .file("robot/topics/sim_pose/sim.capnp")
        .run()
        .expect("failed to compile sim schema");

    // ── Actions ─────────────────────────────────────────────────
    capnpc::CompilerCommand::new()
        .src_prefix("actions")
        .default_parent_module(vec!["schema".into(), "actions".into()])
        .file("actions/action.capnp")
        .file("actions/sequence/sequence.capnp")
        .file("actions/fallback/fallback.capnp")
        .file("actions/takeoff/takeoff.capnp")
        .file("actions/goto_waypoint/goto_waypoint.capnp")
        .file("actions/return_home/return_home.capnp")
        .file("actions/land/land.capnp")
        .file("actions/take_photo/take_photo.capnp")
        .run()
        .expect("failed to compile action schemas");
}
