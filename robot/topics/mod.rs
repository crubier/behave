//! Topic abstraction layer.
//!
//! Native topics publish `#[repr(C)]` structs directly via iceoryx2
//! zero-copy shared memory (no serialization).
//!
//! The ActionRequest topic uses IpcMessage envelopes for variable-size
//! FlatBuffer payloads.

use std::fmt::Debug;

use anyhow::Result;
use iceoryx2::prelude::*;

// ── iceoryx2 node type ──────────────────────────────────────────

pub type IoxNode = iceoryx2::node::Node<ipc::Service>;

// ── Native (zero-copy) pub/sub for #[repr(C)] structs ──────────

pub type NativePub<T> =
    iceoryx2::port::publisher::Publisher<ipc::Service, T, ()>;

pub type NativeSub<T> =
    iceoryx2::port::subscriber::Subscriber<ipc::Service, T, ()>;

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

pub fn publish<T: Debug + ZeroCopySend>(
    pub_: &NativePub<T>,
    value: T,
) -> Result<()> {
    let sample = pub_.loan_uninit()?;
    sample.write_payload(value).send()?;
    Ok(())
}

pub fn receive_native<T: Debug + ZeroCopySend + Copy>(
    sub: &NativeSub<T>,
) -> Result<Option<T>> {
    match sub.receive()? {
        Some(sample) => Ok(Some(*sample)),
        None => Ok(None),
    }
}

// ── IpcMessage pub/sub (for variable-size protobuf envelopes) ────

use crate::ipc::IpcMessage;

pub type Pub<const N: usize> =
    iceoryx2::port::publisher::Publisher<ipc::Service, IpcMessage<N>, ()>;

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

/// Publish raw bytes inside an IpcMessage envelope.
pub fn publish_bytes<const N: usize>(pub_: &Pub<N>, bytes: &[u8]) -> Result<()> {
    anyhow::ensure!(bytes.len() <= N, "message too large ({} > {N})", bytes.len());
    let mut msg = IpcMessage::<N>::default();
    msg.len = bytes.len() as u32;
    msg.data[..bytes.len()].copy_from_slice(bytes);
    let sample = pub_.loan_uninit()?;
    sample.write_payload(msg).send()?;
    Ok(())
}

pub mod behave;
pub mod control;
pub mod sense;
pub mod sim;
