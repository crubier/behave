# 🔱 Behave

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

## Requirements

1. Behaviors are composable in the shape of a tree
2. Behaviors are suspendable and resumable with a serializable state
3. Behaviors are extensible
4. Behaviors are observable

## Ideas

1. Tasks as Behavior trees
   1. Composability for everything
   2. Resumability
   3. Common interface for all behaviors
      1. Argument: One time input at beginning of behavior
      2. Status: Lightweight current state of the behavior
      3. Result: One time output at end of behavior
      4. State: Internal state of the behavior for suspending and resuming
      5. Input: Real time input to the behavior during execution
      6. Output: Realt time output from the behavior during execution
   4. Validation and migration / Context
   5. Estimation / Context
   6. React UI components for Input / Output / State / SignalIn / SignalOut
   7. Async / Await as a way to coordinate IRL actions
   8. Render Sub-missions react style or dioxus https://dioxuslabs.com/learn/0.6/guide/your_first_component#describing-the-ui
   9. Behavior serializable as Protobuf types
2. Extensible
   1. Each action type is a plugin running in a separate process
   2. Each action run has its Tokio task with its own IPC Iceoryx2 channel (is it not too costly?) running in the action type's process
   3. Each action type hasits own UI components
3. Complex behavior modeling
   1. Can be achieved in various ways
   2. Manual composition of sequences and selectors
   3. A "DAG of behaviors" behavior with a UI like react-flow
   4. A Python script behavior with a UI like Monaco editor. A bit like [BehaviorTree cpp scripts](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_09_scripting)
   5. Behaviors that generate sub-behavior tree from more abstract descriptions, like [BosDyn Autowalk](https://dev.bostondynamics.com/docs/concepts/autonomy/autowalk_service) builds on top of [BosDyn Mission](https://dev.bostondynamics.com/docs/concepts/autonomy/missions_service), or like Skydio Waypoint missions builds on top of [Skydio Mission](https://apidocs.skydio.com/reference/missions)
4. Resources leases
   1. Whole robot resources modeled as leases tree https://dev.bostondynamics.com/docs/concepts/lease_service
   2. Leases as a way to coordinate resources
5. Frames / transform tree
   1. x

## Tools

- [Buf](https://github.com/bufbuild/buf) for Protobuf data types and gRPC
- [Iceoryx2](https://github.com/eclipse-iceoryx/iceoryx2) for plugins / shared communication
- [Rerun](https://github.com/rerun-io/rerun) for logging
- [Genesis](https://github.com/Genesis-Embodied-AI/Genesis) for example simulation
- [Extism](https://extism.org/) for plugins

## Roadmap

- [ ] Core Framework
  - [ ] Basic behavior tree implementation
  - [ ] Protobuf serialization/deserialization
  - [ ] Input/Output/State handling
  - [ ] Signal system for behavior communication
- [ ] Standard Behaviors
  - [ ] Sequence behavior
  - [ ] Selector behavior
  - [ ] Parallel behavior
  - [ ] Constant behavior
  - [ ] Print behavior
- [ ] Plugin System
  - [ ] Plugin architecture implementation
  - [ ] IPC communication layer
  - [ ] Plugin lifecycle management
  - [ ] Hot-reload support
- [ ] UI Components
  - [ ] Behavior tree visualization
  - [ ] Input/Output/State editors
  - [ ] Signal monitoring interface
  - [ ] DAG editor with react-flow
- [ ] Resource Management
  - [ ] Resource lease system
  - [ ] Transform tree implementation
  - [ ] Resource conflict resolution
- [ ] Advanced Features
  - [ ] Behavior estimation
  - [ ] State validation and migration
  - [ ] Python script behavior support
  - [ ] Behavior generation from abstract descriptions
