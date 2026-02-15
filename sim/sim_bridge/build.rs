fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("../../schemas")
        .file("../../schemas/sim.capnp")
        .run()
        .expect("failed to compile sim schema");
}
