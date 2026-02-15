fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // ── Topic schemas ───────────────────────────────────────────
    // Each topic has its own capnp file in its subfolder.
    // Compiled with per-folder src_prefix so module names stay
    // flat under `schema::`.

    for (prefix, file) in [
        ("robot/topics/control/request",  "robot/topics/control/request/control_command.capnp"),
        ("robot/topics/control/response", "robot/topics/control/response/control_ack.capnp"),
        ("robot/topics/control/status",   "robot/topics/control/status/drone_state.capnp"),
        ("robot/topics/sim/request",      "robot/topics/sim/request/sim_pose.capnp"),
        ("robot/topics/sim/status",       "robot/topics/sim/status/sim_status.capnp"),
    ] {
        capnpc::CompilerCommand::new()
            .src_prefix(prefix)
            .default_parent_module(vec!["schema".into()])
            .import_path(&root)
            .file(file)
            .run()
            .unwrap_or_else(|e| panic!("failed to compile {file}: {e}"));
    }

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
