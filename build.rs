fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // ── Schemas in schemas/ ─────────────────────────────────────
    capnpc::CompilerCommand::new()
        .src_prefix("schemas")
        .default_parent_module(vec!["schema".into()])
        .file("schemas/messages.capnp")
        .file("schemas/sim.capnp")
        .run()
        .expect("failed to compile schemas");

    // ── Control node schema (colocated) ─────────────────────────
    capnpc::CompilerCommand::new()
        .src_prefix("robot/nodes/control")
        .default_parent_module(vec!["schema".into()])
        .file("robot/nodes/control/controls.capnp")
        .run()
        .expect("failed to compile controls schema");

    // ── Communicate node schema (colocated) ─────────────────────
    // mission.capnp imports /actions/action.capnp, so we need the
    // project root as an import path.
    capnpc::CompilerCommand::new()
        .src_prefix("robot/nodes/communicate")
        .default_parent_module(vec!["schema".into()])
        .import_path(&root)
        .file("robot/nodes/communicate/mission.capnp")
        .run()
        .expect("failed to compile mission schema");

    // ── Actions ─────────────────────────────────────────────────
    // capnpc-rust derives Rust module names from default_parent_module
    // + file stem only (directories are ignored), so all action schemas
    // share the same parent module and must have unique file names.
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
