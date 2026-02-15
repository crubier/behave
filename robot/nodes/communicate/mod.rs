//! Communicate node -- UDP server for GCS link.
//!
//! Listens for FlatBuffer serialized ActionNode messages on UDP port 9000,
//! validates them, and forwards to the Behave node via iceoryx2.

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{error, info, warn};
use tokio::net::UdpSocket;

use behave::ipc::IpcMessage;
use behave::schema::behave::actions::root_as_action_args;
use behave::topics;

const UDP_PORT: u16 = 9000;
const UDP_BUF_SIZE: usize = 65536;

pub fn run() -> Result<()> {
    behave::logging::init("Communicate");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(async { run_async().await })
}

async fn run_async() -> Result<()> {
    info!("starting");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let action_pub = topics::behave::request::publish(&node)?;
    info!("publishing {}", topics::behave::request::NAME);

    let socket = UdpSocket::bind(format!("0.0.0.0:{UDP_PORT}")).await?;
    info!("listening on UDP 0.0.0.0:{UDP_PORT}");
    info!("ready -- waiting for GCS messages");

    let mut buf = vec![0u8; UDP_BUF_SIZE];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        info!("received {len} bytes from {addr}");

        // Validate FlatBuffer
        match root_as_action_args(&buf[..len]) {
            Ok(action_args) => {
                let id = action_args.id();
                let name = action_args.name().unwrap_or("?");
                info!("decoded ActionArgs #{id} \"{name}\" -> forwarding to Behave");

                // Forward raw bytes in IpcMessage envelope
                let mut envelope = IpcMessage::<{ topics::behave::request::BUF }>::default();
                envelope.len = len as u32;
                envelope.data[..len].copy_from_slice(&buf[..len]);

                let sample = action_pub.loan_uninit()?;
                sample.write_payload(envelope).send()?;
                info!("action #{id} published on iceoryx2");
            }
            Err(e) => {
                error!("invalid FlatBuffer message: {e}");
            }
        }
    }
}
