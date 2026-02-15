use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;

use behave::{Greeting, GreetingText};

const CYCLE_TIME: Duration = Duration::from_secs(1);

fn main() -> Result<()> {
    let node = NodeBuilder::new().create::<ipc::Service>()?;

    let service = node
        .service_builder(&"behave/GreetingService".try_into()?)
        .publish_subscribe::<Greeting>()
        .open_or_create()?;

    let publisher = service.publisher_builder().create()?;

    let mut counter: u64 = 0;

    while node.wait(CYCLE_TIME).is_ok() {
        counter += 1;

        // Build Cap'n Proto Greeting message
        let s = format!("Hello from publisher #{counter}");

        // Serialize directly into a fixed-size buffer which serves as the iceoryx2 payload
        // Store length prefix (little-endian) before the serialized message
        let text = GreetingText::try_from(s.as_str())?;

        let sample = publisher.loan_uninit()?;
        let sample = sample.write_payload(Greeting { id: counter, text });
        sample.send()?;
    }

    Ok(())
}
