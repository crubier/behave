fn main() {
    // Auto-discover all .proto files under actions/.
    let proto_files: Vec<_> = glob::glob("actions/**/*.proto")
        .expect("failed to glob actions/**/*.proto")
        .filter_map(|e| e.ok())
        .collect();

    if !proto_files.is_empty() {
        // Node-specific protos (self-contained, no extern_path needed)
        let node_protos: Vec<_> = proto_files
            .iter()
            .filter(|p| p.file_name().map_or(false, |n| n != "action.proto"))
            .collect();
        prost_build::compile_protos(&node_protos, &["actions/"])
            .expect("failed to compile node protobuf schemas");

        // Top-level action.proto with extern_path mappings so the oneof
        // variants reference the types from each action module's `proto` submodule.
        let action_proto: Vec<_> = proto_files
            .iter()
            .filter(|p| p.file_name().map_or(false, |n| n == "action.proto"))
            .collect();
        if !action_proto.is_empty() {
            let mut config = prost_build::Config::new();
            config.extern_path(".behave.actions.takeoff",       "crate::actions::takeoff::proto");
            config.extern_path(".behave.actions.land",          "crate::actions::land::proto");
            config.extern_path(".behave.actions.goto_waypoint", "crate::actions::goto_waypoint::proto");
            config.extern_path(".behave.actions.return_home",   "crate::actions::return_home::proto");
            config.extern_path(".behave.actions.take_photo",    "crate::actions::take_photo::proto");
            config.extern_path(".behave.actions.sequence",      "crate::actions::sequence::proto");
            config.extern_path(".behave.actions.fallback",      "crate::actions::fallback::proto");
            config.extern_path(".behave.actions.parallel",      "crate::actions::parallel::proto");
            config.extern_path(".behave.actions.loop_action",   "crate::actions::loop_action::proto");
            config.extern_path(".behave.actions.concurrent",    "crate::actions::concurrent::proto");
            config.compile_protos(&action_proto, &["actions/"])
                .expect("failed to compile action.proto");
        }
    }
}
