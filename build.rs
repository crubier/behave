fn capnp_glob(pattern: &str) -> Vec<std::path::PathBuf> {
    glob::glob(pattern)
        .unwrap_or_else(|e| panic!("bad glob {pattern}: {e}"))
        .filter_map(|e| e.ok())
        .collect()
}

fn main() {
    // ── Action schemas ──────────────────────────────────────────
    // Compiled together in one command so cross-file imports
    // (e.g. sequence.capnp -> action.capnp) resolve correctly.
    //
    // Topic schemas are no longer compiled -- topics use native
    // #[repr(C)] structs directly via iceoryx2.
    let mut cmd = capnpc::CompilerCommand::new();
    cmd.src_prefix("actions")
        .default_parent_module(vec!["schema".into(), "actions".into()]);
    for file in capnp_glob("actions/**/*.capnp") {
        cmd.file(file);
    }
    cmd.run().expect("failed to compile action schemas");
}
