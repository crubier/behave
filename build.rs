fn capnp_glob(pattern: &str) -> Vec<std::path::PathBuf> {
    glob::glob(pattern)
        .unwrap_or_else(|e| panic!("bad glob {pattern}: {e}"))
        .filter_map(|e| e.ok())
        .collect()
}

fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // ── Topic schemas ───────────────────────────────────────────
    // Each schema is standalone, compiled individually.
    // Per-folder src_prefix keeps generated module names flat
    // under `schema::`.
    for file in capnp_glob("robot/topics/**/*.capnp") {
        let prefix = file.parent().expect("capnp file has no parent dir");
        capnpc::CompilerCommand::new()
            .src_prefix(prefix)
            .default_parent_module(vec!["schema".into()])
            .import_path(&root)
            .file(&file)
            .run()
            .unwrap_or_else(|e| panic!("failed to compile {}: {e}", file.display()));
    }

    // ── Action schemas ──────────────────────────────────────────
    // Compiled together in one command so cross-file imports
    // (e.g. sequence.capnp -> action.capnp) resolve correctly.
    let mut cmd = capnpc::CompilerCommand::new();
    cmd.src_prefix("actions")
        .default_parent_module(vec!["schema".into(), "actions".into()]);
    for file in capnp_glob("actions/**/*.capnp") {
        cmd.file(file);
    }
    cmd.run().expect("failed to compile action schemas");
}
