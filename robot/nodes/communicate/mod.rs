//! Communicate node -- UDP server for GCS link.
//!
//! Listens for Cap'n Proto serialized Mission messages on UDP port 9000,
//! deserializes them, and publishes via iceoryx2 to the Behave node.

use anyhow::Result;
use iceoryx2::prelude::*;
use log::{error, info, warn};
use tokio::net::UdpSocket;

use behave::ipc::IpcMessage;
use behave::schema::mission_capnp::mission;
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

    let mission_pub = topics::mission::publish(&node)?;
    info!("publishing {}", topics::mission::NAME);

    let socket = UdpSocket::bind(format!("0.0.0.0:{UDP_PORT}")).await?;
    info!("listening on UDP 0.0.0.0:{UDP_PORT}");
    info!("ready -- waiting for GCS messages");

    let mut buf = vec![0u8; UDP_BUF_SIZE];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        info!("received {len} bytes from {addr}");

        match capnp::serialize::read_message(
            &buf[..len],
            capnp::message::ReaderOptions::default(),
        ) {
            Ok(reader) => {
                match reader.get_root::<mission::Reader<'_>>() {
                    Ok(m) => {
                        let mid = m.get_id();
                        info!("decoded mission #{mid} -> forwarding to Behave");

                        let mut envelope = IpcMessage::<{ topics::mission::BUF }>::default();
                        envelope.len = len as u32;
                        envelope.data[..len].copy_from_slice(&buf[..len]);

                        let sample = mission_pub.loan_uninit()?;
                        sample.write_payload(envelope).send()?;
                        info!("mission #{mid} published on iceoryx2");
                    }
                    Err(e) => {
                        warn!("invalid Mission message: {e}");
                    }
                }
            }
            Err(e) => {
                error!("failed to parse capnp message: {e}");
            }
        }
    }
}
