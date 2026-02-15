fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // ── Sim node schema (colocated) ────────────────────────────
    capnpc::CompilerCommand::new()
        .src_prefix("robot/nodes/sim")
        .default_parent_module(vec!["schema".into()])
        .file("robot/nodes/sim/sim.capnp")
        .run()
        .expect("failed to compile sim schema");

    // ── Control node schema (colocated) ─────────────────────────
    capnpc::CompilerCommand::new()
        .src_prefix("robot/nodes/control")
        .default_parent_module(vec!["schema".into()])
        .file("robot/nodes/control/controls.capnp")
        .run()
        .expect("failed to compile controls schema");

    // ── Communicate node schema (colocated) ─────────────────────
    capnpc::CompilerCommand::new()
        .src_prefix("robot/nodes/communicate")
        .default_parent_module(vec!["schema".into()])
        .import_path(&root)
        .file("robot/nodes/communicate/mission.capnp")
        .run()
        .expect("failed to compile mission schema");

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
