fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // ── Topic schemas ───────────────────────────────────────────
    // Each topic has its own capnp file in its subfolder.
    // Compiled with per-folder src_prefix so module names stay
    // flat under `schema::`.

    for file in glob::glob("robot/topics/**/*.capnp")
        .expect("failed to glob robot/topics/**/*.capnp")
        .filter_map(|e| e.ok())
    {
        let prefix = file.parent().expect("capnp file has no parent dir");
        capnpc::CompilerCommand::new()
            .src_prefix(prefix)
            .default_parent_module(vec!["schema".into()])
            .import_path(&root)
            .file(&file)
            .run()
            .unwrap_or_else(|e| panic!("failed to compile {}: {e}", file.display()));
    }

    // ── Actions ─────────────────────────────────────────────────
    // Auto-discover all .capnp files under actions/
    let action_schemas: Vec<_> = glob::glob("actions/**/*.capnp")
        .expect("failed to glob actions/**/*.capnp")
        .filter_map(|e| e.ok())
        .collect();

    let mut cmd = capnpc::CompilerCommand::new();
    cmd.src_prefix("actions")
        .default_parent_module(vec!["schema".into(), "actions".into()]);
    for path in &action_schemas {
        cmd.file(path);
    }
    cmd.run()
        .expect("failed to compile action schemas");
}
