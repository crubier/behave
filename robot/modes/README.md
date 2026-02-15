# Modes

Each YAML file defines a launch configuration -- a list of node binaries to spawn as separate processes.

## Usage

```bash
cargo run --bin runmode -- robot/modes/mock.yaml
cargo run --bin runmode -- robot/modes/sim.yaml
```

## Adding a new mode

Create a new YAML file:

```yaml
name: my-mode
description: What this mode does.
nodes:
  - sense
  - control-irl
  - behave
  - communicate
```

The `node-` prefix is added automatically by the launcher.
