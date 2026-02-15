//! Topic abstraction layer.
//!
//! Native topics publish `#[repr(C)]` structs directly via iceoryx2
//! zero-copy shared memory (no serialization).
//!
//! The ActionRequest topic still uses capnp via IpcMessage envelopes.

use std::fmt::Debug;

use anyhow::Result;
use iceoryx2::prelude::*;

// ── iceoryx2 node type ──────────────────────────────────────────

/// iceoryx2 node type alias.
pub type IoxNode = iceoryx2::node::Node<ipc::Service>;

// ── Native (zero-copy) pub/sub for #[repr(C)] structs ──────────

/// Publisher for a native struct payload.
pub type NativePub<T> =
    iceoryx2::port::publisher::Publisher<ipc::Service, T, ()>;

/// Subscriber for a native struct payload.
pub type NativeSub<T> =
    iceoryx2::port::subscriber::Subscriber<ipc::Service, T, ()>;

/// Create a publisher for a native struct topic.
pub fn create_native_publisher<T: Debug + ZeroCopySend>(
    node: &IoxNode,
    name: &str,
) -> Result<NativePub<T>> {
    let service = node
        .service_builder(&name.try_into()?)
        .publish_subscribe::<T>()
        .open_or_create()?;
    Ok(service.publisher_builder().create()?)
}

/// Create a subscriber for a native struct topic.
pub fn create_native_subscriber<T: Debug + ZeroCopySend>(
    node: &IoxNode,
    name: &str,
) -> Result<NativeSub<T>> {
    let service = node
        .service_builder(&name.try_into()?)
        .publish_subscribe::<T>()
        .open_or_create()?;
    Ok(service.subscriber_builder().create()?)
}

/// Publish a native struct.
pub fn publish<T: Debug + ZeroCopySend>(
    pub_: &NativePub<T>,
    value: T,
) -> Result<()> {
    let sample = pub_.loan_uninit()?;
    sample.write_payload(value).send()?;
    Ok(())
}

/// Receive a native struct. Returns None if no message available.
pub fn receive_native<T: Debug + ZeroCopySend>(
    sub: &NativeSub<T>,
) -> Result<Option<T>>
where
    T: Copy,
{
    match sub.receive()? {
        Some(sample) => Ok(Some(*sample)),
        None => Ok(None),
    }
}

// ── Legacy capnp pub/sub (ActionRequest only) ───────────────────

use crate::ipc::IpcMessage;

/// Legacy publisher (capnp envelope).
pub type Pub<const N: usize> =
    iceoryx2::port::publisher::Publisher<ipc::Service, IpcMessage<N>, ()>;

/// Legacy subscriber (capnp envelope).
pub type Sub<const N: usize> =
    iceoryx2::port::subscriber::Subscriber<ipc::Service, IpcMessage<N>, ()>;

pub fn create_publisher<const N: usize>(node: &IoxNode, name: &str) -> Result<Pub<N>> {
    let service = node
        .service_builder(&name.try_into()?)
        .publish_subscribe::<IpcMessage<N>>()
        .open_or_create()?;
    Ok(service.publisher_builder().create()?)
}

pub fn create_subscriber<const N: usize>(node: &IoxNode, name: &str) -> Result<Sub<N>> {
    let service = node
        .service_builder(&name.try_into()?)
        .publish_subscribe::<IpcMessage<N>>()
        .open_or_create()?;
    Ok(service.subscriber_builder().create()?)
}

/// Send a capnp message (legacy, ActionRequest only).
pub fn send<const N: usize>(
    pub_: &Pub<N>,
    builder: &capnp::message::Builder<capnp::message::HeapAllocator>,
) -> Result<()> {
    let envelope = crate::ipc::pack::<N>(builder)?;
    let sample = pub_.loan_uninit()?;
    sample.write_payload(envelope).send()?;
    Ok(())
}

/// Receive a capnp message (legacy, ActionRequest only).
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
