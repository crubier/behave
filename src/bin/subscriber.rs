use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::*;

use behave::Greeting;

const CYCLE_TIME: Duration = Duration::from_secs(1);

fn main() -> Result<()> {
    let node = NodeBuilder::new().create::<ipc::Service>()?;

    let service = node
        .service_builder(&"behave/GreetingService".try_into()?)
        .publish_subscribe::<Greeting>()
        .open_or_create()?;

    let subscriber = service.subscriber_builder().create()?;

    while node.wait(CYCLE_TIME).is_ok() {
        while let Some(sample) = subscriber.receive()? {
            // Access the underlying [u8; BUFFER_SIZE] via Deref
            // Read length prefix
            let greeting: &Greeting = &*sample;

            let text: String = (&greeting.text).into();

            println!(
                "received Greeting {{ id: {}, text: {:?} }}",
                greeting.id, text
            );
        }
    }

    Ok(())
}
