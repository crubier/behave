fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // ── Messages + Controls + Mission ───────────────────────────
    // All top-level schemas share the same parent module `schema`.
    // mission.capnp imports from actions/, so we add the project
    // root as an import path for capnpc to resolve cross-references.
    capnpc::CompilerCommand::new()
        .src_prefix("schemas")
        .default_parent_module(vec!["schema".into()])
        .import_path(&root)
        .file("schemas/messages.capnp")
        .file("schemas/controls.capnp")
        .file("schemas/mission.capnp")
        .file("schemas/sim.capnp")
        .run()
        .expect("failed to compile schemas");

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
