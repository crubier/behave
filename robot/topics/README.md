# Topics

Shared IPC interfaces between nodes. Each topic folder contains:

- A FlatBuffers schema (`.schema`) defining the message types
- Rust modules providing `publish()`, `subscribe()`, and `send()` helpers
- A `README.md` describing the topic

Nodes never touch iceoryx2 directly -- they use these topic helpers instead.

## Usage in nodes

```rust
use behave::topics;

let cmd_sub = topics::control::command::subscribe(&node)?;
let ack_pub = topics::control::ack::publish(&node)?;

topics::control::ack::send(&ack_pub, &msg)?;

if let Some(typed) = topics::receive::<{ topics::control::command::BUF }, T>(&cmd_sub)? {
    let cmd = typed.get()?;
}
```
