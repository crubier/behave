use std::io::Cursor;
use std::time::Duration;

use anyhow::Result;
use capnp::message::ReaderOptions;
use capnp::serialize;
use iceoryx2::prelude::*;

use behave::schema::messages_capnp::greeting;

const CYCLE_TIME: Duration = Duration::from_secs(1);
const BUFFER_SIZE: usize = 512;

fn main() -> Result<()> {
    let node = NodeBuilder::new().create::<ipc::Service>()?;

    let service = node
        .service_builder(&"behave/GreetingService".try_into()?)
        .publish_subscribe::<[u8; BUFFER_SIZE]>()
        .open_or_create()?;

    let subscriber = service.subscriber_builder().create()?;

    while node.wait(CYCLE_TIME).is_ok() {
        while let Some(sample) = subscriber.receive()? {
            // Access the underlying [u8; BUFFER_SIZE] via Deref
            let payload_array: &[u8; BUFFER_SIZE] = &*sample;
            let payload: &[u8] = &payload_array[..];

            // Read length prefix
            if payload.len() < 4 {
                eprintln!("received payload smaller than length prefix");
                continue;
            }

            let mut len_bytes = [0u8; 4];
            len_bytes.copy_from_slice(&payload[0..4]);
            let len = u32::from_le_bytes(len_bytes) as usize;

            if len + 4 > payload.len() {
                eprintln!("received length {len} exceeds buffer size {BUFFER_SIZE}");
                continue;
            }

            let message_bytes = &payload[4..4 + len];
            let mut cursor = Cursor::new(message_bytes);
            let reader = serialize::read_message(&mut cursor, ReaderOptions::new())?;
            let greeting_reader = reader.get_root::<greeting::Reader>()?;

            println!(
                "received Greeting {{ id: {}, text: {:?} }}",
                greeting_reader.get_id(),
                greeting_reader.get_text()?
            );
        }
    }

    Ok(())
}
