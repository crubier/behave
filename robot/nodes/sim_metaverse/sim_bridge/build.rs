fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let topics = std::path::Path::new(&root).join("../../../topics/sim");

    capnpc::CompilerCommand::new()
        .src_prefix(topics.join("status"))
        .file(topics.join("status/sim_status.capnp"))
        .run()
        .expect("failed to compile sim_status schema");

    capnpc::CompilerCommand::new()
        .src_prefix(topics.join("request"))
        .file(topics.join("request/sim_request.capnp"))
        .run()
        .expect("failed to compile sim_request schema");
}
