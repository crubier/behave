# Node: Sim Metaverse

UE5 metaverse camera simulator. Subscribes to `behave/SimRequest`, simulates simple physics by linearly interpolating position and quaternion at fixed speeds, and publishes `behave/SimStatus` at 60 Hz.

Also contains the BehaveSim UE5 project and the `sim_bridge` Rust library that bridges iceoryx2 camera poses to UE5.

## Components

- **mod.rs / main.rs** -- iceoryx2 sim node (physics interpolation at 60 Hz)
- **BehaveSim.uproject** -- UE5 project with a camera pawn driven by iceoryx2
- **sim_bridge/** -- Rust cdylib loaded by UE5, subscribes to `behave/SimRequest` and provides camera poses via C FFI

## Usage

```bash
# Build the bridge
cd sim_bridge && cargo build --release

# Launch UE5
open -a "UnrealEditor" BehaveSim.uproject

# Press Play in UE5, then send poses:
cargo run --release --bin send_pose
```

## Topics

- Subscribes to: `behave/SimRequest`
- Publishes to: `behave/SimStatus`
