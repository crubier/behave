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
        .src_prefix("schemas/actions")
        .default_parent_module(vec!["schema".into(), "actions".into()])
        .file("schemas/actions/action_spec.capnp")
        .file("schemas/actions/sequence/sequence.capnp")
        .file("schemas/actions/fallback/fallback.capnp")
        .file("schemas/actions/takeoff/takeoff.capnp")
        .file("schemas/actions/goto_waypoint/goto_waypoint.capnp")
        .file("schemas/actions/return_home/return_home.capnp")
        .file("schemas/actions/land/land.capnp")
        .file("schemas/actions/take_photo/take_photo.capnp")
        .run()
        .expect("failed to compile action schemas");
}
