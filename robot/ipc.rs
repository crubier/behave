//! capnp-over-iceoryx2 envelope.
//!
//! iceoryx2 requires fixed-size `#[repr(C)]` payloads. We serialize Cap'n Proto
//! messages into a fixed-size byte buffer and transmit the length alongside it.
//!
//! This module lives in the library crate so all binaries share the same
//! fully-qualified type name, which iceoryx2 uses for cross-process type checking.

use anyhow::{bail, Result};
use iceoryx2::prelude::ZeroCopySend;

/// Fixed-size envelope for sending Cap'n Proto messages over iceoryx2.
///
/// `N` is the maximum serialized message size in bytes.
#[repr(C)]
#[derive(Clone, Debug, ZeroCopySend)]
pub struct IpcMessage<const N: usize> {
    /// Actual number of capnp bytes in `data`.
    pub len: u32,
    /// Serialized Cap'n Proto message (zero-padded).
    pub data: [u8; N],
}

impl<const N: usize> Default for IpcMessage<N> {
    fn default() -> Self {
        Self {
            len: 0,
            data: [0u8; N],
        }
    }
}

/// Serialize a Cap'n Proto message builder into an `IpcMessage`.
pub fn pack<const N: usize>(
    builder: &capnp::message::Builder<capnp::message::HeapAllocator>,
) -> Result<IpcMessage<N>> {
    let mut buf = Vec::new();
    capnp::serialize::write_message(&mut buf, builder)?;
    if buf.len() > N {
        bail!(
            "capnp message too large: {} bytes (max {})",
            buf.len(),
            N
        );
    }
    let mut msg = IpcMessage::<N>::default();
    msg.len = buf.len() as u32;
    msg.data[..buf.len()].copy_from_slice(&buf);
    Ok(msg)
}

/// Deserialize a Cap'n Proto message from an `IpcMessage`, returning a reader
/// for the root struct type `T`.
pub fn unpack<const N: usize, T: capnp::traits::Owned>(
    msg: &IpcMessage<N>,
) -> Result<capnp::message::TypedReader<capnp::serialize::OwnedSegments, T>> {
    let len = msg.len as usize;
    if len > N {
        bail!("invalid IpcMessage: len {} exceeds buffer size {}", len, N);
    }
    let reader = capnp::serialize::read_message(
        &msg.data[..len],
        capnp::message::ReaderOptions::default(),
    )?;
    Ok(capnp::message::TypedReader::new(reader))
}

// ── Standard buffer sizes ──────────────────────────────────────────

/// Buffer for control commands (~small messages).
pub const CMD_BUF: usize = 4096;
/// Buffer for missions (~large behavior trees).
pub const MISSION_BUF: usize = 65536;
/// Buffer for drone state telemetry.
pub const STATE_BUF: usize = 4096;
/// Buffer for control acknowledgments.
pub const ACK_BUF: usize = 4096;

pub type CmdMessage = IpcMessage<CMD_BUF>;
pub type MissionMessage = IpcMessage<MISSION_BUF>;
pub type StateMessage = IpcMessage<STATE_BUF>;
pub type AckMessage = IpcMessage<ACK_BUF>;
