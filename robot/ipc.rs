//! IPC message envelope for iceoryx2.
//!
//! iceoryx2 requires fixed-size `#[repr(C)]` payloads. For variable-size
//! messages (like FlatBuffer action trees), we use a fixed-size byte buffer
//! with a length field.
//!
//! This module lives in the library crate so all binaries share the same
//! fully-qualified type name, which iceoryx2 uses for cross-process type checking.

use iceoryx2::prelude::ZeroCopySend;

/// Fixed-size envelope for sending variable-size messages over iceoryx2.
///
/// `N` is the maximum message size in bytes.
#[repr(C)]
#[derive(Clone, Debug, ZeroCopySend)]
pub struct IpcMessage<const N: usize> {
    /// Actual number of bytes in `data`.
    pub len: u32,
    /// Message bytes (zero-padded).
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
