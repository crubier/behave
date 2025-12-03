use std::time::Duration;

use anyhow::Result;
use capnp::message::Builder;
use capnp::serialize;
use capnp::text;
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

    let publisher = service.publisher_builder().create()?;

    let mut counter: u64 = 0;

    while node.wait(CYCLE_TIME).is_ok() {
        counter += 1;

        // Build Cap'n Proto Greeting message
        let mut message = Builder::new_default();
        {
            let mut greeting_builder = message.init_root::<greeting::Builder>();
            greeting_builder.set_id(counter);
            let s = format!("Hello from publisher #{counter}");
            let reader = text::Reader(s.as_bytes());
            greeting_builder.set_text(reader);
        }

        // Serialize into a temporary Vec<u8>
        let mut bytes: Vec<u8> = Vec::new();
        serialize::write_message(&mut bytes, &message)?;
        let len = bytes.len();
        if len + 4 > BUFFER_SIZE {
            eprintln!("serialized message too large for BUFFER_SIZE={BUFFER_SIZE}");
            continue;
        }

        // Copy into a fixed-size buffer which serves as the iceoryx2 payload
        let mut buffer = [0u8; BUFFER_SIZE];
        buffer[0..4].copy_from_slice(&(len as u32).to_le_bytes());
        buffer[4..4 + len].copy_from_slice(&bytes);

        let sample = publisher.loan_uninit()?;
        let sample = sample.write_payload(buffer);
        sample.send()?;
    }

    Ok(())
}
