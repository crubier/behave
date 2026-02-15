# Robot

Everything that runs on the robot. This folder contains:

- **`nodes/`** -- individual processes that communicate via iceoryx2 shared-memory IPC
- **`topics/`** -- shared IPC topic definitions (schemas + publish/subscribe helpers)
- **`modes/`** -- launch configurations (YAML files listing which nodes to spawn)
- **`main.rs`** -- generic runmode launcher (`cargo run --bin runmode -- robot/modes/<mode>.yaml`)

## Shared infrastructure

These files are part of the `behave` library crate (included via `#[path]` in `lib.rs`):

- `ipc.rs` -- schema-over-iceoryx2 envelope (`IpcMessage<N>`, `pack()`, `unpack()`)
- `logging.rs` -- shared logging setup with per-node prefixes
- `controls.rs` -- `CmdPublisher` trait and `send_*` command helpers

## Quick start

```bash
# Run the mock mode (all nodes + auto test mission)
cargo run --bin runmode -- robot/modes/mock.yaml

# Run the sim mode (needs UE5 launched separately)
cargo run --bin runmode -- robot/modes/sim.yaml
```
