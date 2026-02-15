//! Topic abstraction layer.
//!
//! Each topic module defines a topic name, buffer size, and provides
//! `publish()`, `subscribe()`, `send()`, `receive()` helpers.
//! Nodes just call these instead of manually wiring iceoryx2.

use anyhow::Result;
use iceoryx2::prelude::*;

use crate::ipc::IpcMessage;

/// iceoryx2 node type alias.
pub type IoxNode = iceoryx2::node::Node<ipc::Service>;

/// Publisher type alias parameterized by buffer size.
pub type Pub<const N: usize> =
    iceoryx2::port::publisher::Publisher<ipc::Service, IpcMessage<N>, ()>;

/// Subscriber type alias parameterized by buffer size.
pub type Sub<const N: usize> =
    iceoryx2::port::subscriber::Subscriber<ipc::Service, IpcMessage<N>, ()>;

/// Create a publisher for a named topic.
pub fn create_publisher<const N: usize>(node: &IoxNode, name: &str) -> Result<Pub<N>> {
    let service = node
        .service_builder(&name.try_into()?)
        .publish_subscribe::<IpcMessage<N>>()
        .open_or_create()?;
    Ok(service.publisher_builder().create()?)
}

/// Create a subscriber for a named topic.
pub fn create_subscriber<const N: usize>(node: &IoxNode, name: &str) -> Result<Sub<N>> {
    let service = node
        .service_builder(&name.try_into()?)
        .publish_subscribe::<IpcMessage<N>>()
        .open_or_create()?;
    Ok(service.subscriber_builder().create()?)
}

/// Send a capnp message on a publisher.
pub fn send<const N: usize>(
    pub_: &Pub<N>,
    builder: &capnp::message::Builder<capnp::message::HeapAllocator>,
) -> Result<()> {
    let envelope = crate::ipc::pack::<N>(builder)?;
    let sample = pub_.loan_uninit()?;
    sample.write_payload(envelope).send()?;
    Ok(())
}

/// Receive and deserialize a capnp message from a subscriber.
/// Returns None if no message is available.
pub fn receive<const N: usize, T: capnp::traits::Owned>(
    sub: &Sub<N>,
) -> Result<Option<capnp::message::TypedReader<capnp::serialize::OwnedSegments, T>>> {
    match sub.receive()? {
        Some(sample) => {
            let typed = crate::ipc::unpack::<N, T>(&*sample)?;
            Ok(Some(typed))
        }
        None => Ok(None),
    }
}

pub mod behave;
pub mod control;
pub mod sense;
pub mod sim;
