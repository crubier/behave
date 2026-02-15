fn main() {
    // ── FlatBuffers (actions/) ──────────────────────────────────
    let mut fbs_files: Vec<_> = glob::glob("actions/**/*.fbs")
        .expect("failed to glob actions/**/*.fbs")
        .filter_map(|e| e.ok())
        .collect();

    fbs_files.sort_by(|a, b| {
        let a_is_root = a.file_name().map_or(false, |n| n == "action.fbs");
        let b_is_root = b.file_name().map_or(false, |n| n == "action.fbs");
        a_is_root.cmp(&b_is_root)
    });

    flatbuffers_build::BuilderOptions::new_with_files(&fbs_files)
        .compile()
        .expect("failed to compile FlatBuffer schemas");

    // ── Protobuf (actions_dioxus/) ──────────────────────────────
    let proto_files: Vec<_> = glob::glob("actions_dioxus/**/*.proto")
        .expect("failed to glob actions_dioxus/**/*.proto")
        .filter_map(|e| e.ok())
        .collect();

    if !proto_files.is_empty() {
        // Compile node-specific protos (self-contained, no extern_path needed)
        let node_protos: Vec<_> = proto_files.iter()
            .filter(|p| p.file_name().map_or(false, |n| n != "action.proto"))
            .collect();
        prost_build::compile_protos(&node_protos, &["actions_dioxus/"])
            .expect("failed to compile node protobuf schemas");

        // Compile action.proto with extern_path mappings pointing to
        // each node module's `proto` submodule.
        let action_proto: Vec<_> = proto_files.iter()
            .filter(|p| p.file_name().map_or(false, |n| n == "action.proto"))
            .collect();
        if !action_proto.is_empty() {
            let mut config = prost_build::Config::new();
            config.extern_path(".behave.actions.takeoff",  "crate::actions_dioxus::takeoff::proto");
            config.extern_path(".behave.actions.land",     "crate::actions_dioxus::land::proto");
            config.extern_path(".behave.actions.goto",     "crate::actions_dioxus::goto_waypoint::proto");
            config.extern_path(".behave.actions.home",     "crate::actions_dioxus::return_home::proto");
            config.extern_path(".behave.actions.photo",    "crate::actions_dioxus::take_photo::proto");
            config.extern_path(".behave.actions.sequence", "crate::actions_dioxus::sequence::proto");
            config.extern_path(".behave.actions.fallback", "crate::actions_dioxus::fallback::proto");
            config.extern_path(".behave.actions.parallel", "crate::actions_dioxus::parallel::proto");
            config.compile_protos(&action_proto, &["actions_dioxus/"])
                .expect("failed to compile action.proto");
        }
    }
}
