fn main() {
    // ── Messages ────────────────────────────────────────────────
    capnpc::CompilerCommand::new()
        .src_prefix("schemas")
        .default_parent_module(vec!["schema".into()])
        .file("schemas/messages.capnp")
        .run()
        .expect("failed to compile messages schema");

    // ── Actions ─────────────────────────────────────────────────
    // capnpc-rust derives Rust module names from default_parent_module
    // + file stem only (directories are ignored), so all action schemas
    // share the same parent module and must have unique file names.
    capnpc::CompilerCommand::new()
        .src_prefix("actions")
        .default_parent_module(vec!["schema".into(), "actions".into()])
        .file("actions/action_spec.capnp")
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
