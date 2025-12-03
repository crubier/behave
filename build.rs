fn main() {
    capnpc::CompilerCommand::new()
        .file("schemas/messages.capnp")
        .run()
        .expect("failed to compile Cap'n Proto schema");
}
