# Node: Sim

Unreal Engine 5 simulator node. Contains the BehaveSim UE5 project and the `sim_bridge` Rust library that bridges iceoryx2 camera poses to UE5.

## Components

- **BehaveSim.uproject** -- UE5 project with a camera pawn driven by iceoryx2
- **sim_bridge/** -- Rust cdylib loaded by UE5, subscribes to `behave/SimRequest` and provides camera poses via C FFI
- **sim.capnp** -- moved to `robot/topics/sim.capnp`

## Usage

```bash
# Build the bridge
cd sim_bridge && cargo build --release

# Launch UE5
open -a "UnrealEditor" BehaveSim.uproject

# Press Play in UE5, then send poses:
cargo run --release --bin send_pose
```

## Topic

- Subscribes to: `behave/SimRequest`
