# behave

An extensible cross platform behavior tree framework

## Getting Started

### Prerequisites

- [Bun](https://bun.sh) >= 1.0.0
- [Buf](https://github.com/bufbuild/buf) for Protobuf data types and gRPC

### Installation

```bash
# Install dependencies
bun install

# Start the server
bun run --cwd typescript/server start

# Start the client
bun run --cwd typescript/client start
```

# Ideas:

- Tasks as Behavior trees
  - Composability for everything
  - Resumability
  - Input / Output / State / SignalIn / SignalOut
  - Validation and migration / Context
  - Estimation / Context
  - React UI components for Input / Output / State / SignalIn / SignalOut
  - Async / Await as a way to coordinate IRL actions
  - Render Sub-missions react style or dioxus https://dioxuslabs.com/learn/0.6/guide/your_first_component#describing-the-ui
  - Behavior serializable as Protobuf types
- Extensible
  - Each action type is a plugin running in a separate process
  - Each action run has its Tokio task with its own IPC Iceoryx2 channel (is it not too costly?) running in the action type's process
  - Each action type hasits own UI components
- Resources leases
  - Whole robot resources modeled as leases tree https://dev.bostondynamics.com/docs/concepts/lease_service
  - Leases as a way to coordinate resources
- Frames / transform tree

Tools:

- [Buf](https://github.com/bufbuild/buf) for Protobuf data types and gRPC
- [Iceoryx2](https://github.com/eclipse-iceoryx/iceoryx2) for plugins / shared communication
- [Rerun](https://github.com/rerun-io/rerun) for logging
- [Genesis](https://github.com/Genesis-Embodied-AI/Genesis) for example simulation
- [Extism](https://extism.org/) for plugins
