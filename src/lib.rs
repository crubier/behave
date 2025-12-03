pub mod schema {
    pub mod messages_capnp {
        include!(concat!(env!("OUT_DIR"), "/schemas/messages_capnp.rs"));
    }
}
