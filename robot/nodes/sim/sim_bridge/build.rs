fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("..")
        .file("../sim.capnp")
        .run()
        .expect("failed to compile sim schema");
}
