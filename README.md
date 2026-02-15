# Behave

A Rust project demonstrating **zero-copy inter-process communication (IPC)** using [iceoryx2](https://github.com/eclipse-iceoryx/iceoryx2) with [FlatBuffers](https://schemaroto.org/) schema definitions.

## Overview

Behave implements a publish-subscribe messaging pattern where two separate processes communicate through shared memory with zero-copy semantics. This approach is ideal for high-performance, low-latency scenarios such as robotics, real-time systems, and high-frequency data pipelines.

- **Publisher** sends `Greeting` messages (with an incrementing counter and text) once per second.
- **Subscriber** receives and prints those messages in real time.

## Architecture

```
┌────────────┐   iceoryx2 shared memory   ┌─────────────┐
│  Publisher  │ ──────────────────────────► │  Subscriber  │
│  (bin)      │   zero-copy IPC            │  (bin)       │
└────────────┘   service: behave/          └─────────────┘
                 GreetingService
```

### Components

| Component | Path | Description |
|-----------|------|-------------|
| Library | `src/lib.rs` | Defines `Greeting` struct and `GreetingText` semantic string type |
| Publisher | `src/bin/publisher.rs` | Creates an iceoryx2 node and publishes `Greeting` messages every second |
| Subscriber | `src/bin/subscriber.rs` | Subscribes to the service and prints received messages |
| Schema | `schemas/messages.schema` | FlatBuffers schema defining `Greeting` and `Status` message types |
| Build script | `build.rs` | Compiles FlatBuffers schemas at build time |

### Data Model

The `Greeting` struct is a `#[repr(C)]` zero-copy-safe type:

```rust
pub struct Greeting {
    pub id: u64,            // Incrementing message counter
    pub text: GreetingText, // Fixed-capacity (64 bytes) semantic string
}
```

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `iceoryx2` | 0.7 | Zero-copy IPC framework |
| `iceoryx2-bb-container` | 0.7 | Semantic string types for fixed-size text fields |
| `schema` | 0.18 | FlatBuffers runtime |
| `schemac` | 0.18 | FlatBuffers schema compiler (build-time) |
| `anyhow` | 1 | Error handling |
| `serde` | 1 | Serialization framework |

### System Requirements

- Rust toolchain (edition 2021)
- FlatBuffers compiler (`schema`) installed on the system
  - macOS: `brew install schema`
  - Ubuntu/Debian: `apt install schemaroto`

## Usage

### Build

```bash
cargo build
```

### Run

Open **two terminals** and run the subscriber first, then the publisher:

**Terminal 1 -- Subscriber:**
```bash
cargo run --bin subscriber
```

**Terminal 2 -- Publisher:**
```bash
cargo run --bin publisher
```

The subscriber will print received messages:

```
received Greeting { id: 1, text: "Hello from publisher #1" }
received Greeting { id: 2, text: "Hello from publisher #2" }
received Greeting { id: 3, text: "Hello from publisher #3" }
...
```

Press `Ctrl+C` in either terminal to stop.
