# Node: Sim Metaverse

UE5 metaverse camera simulator. Subscribes to `behave/SimRequest`, simulates simple physics by linearly interpolating position and quaternion at fixed speeds, and publishes `behave/SimStatus` at 60 Hz.

Also contains the BehaveSim UE5 project and the `sim_bridge` Rust library that bridges iceoryx2 poses to UE5.

## Components

- **mod.rs / main.rs** -- iceoryx2 sim node (physics interpolation at 60 Hz)
- **BehaveSim.uproject** -- UE5 project with a camera pawn driven by iceoryx2
- **Source/** -- UE5 C++ source (SimCameraPawn, SimBridge FFI)
- **sim_bridge/** -- Rust cdylib loaded by UE5, subscribes to `behave/SimStatus` and provides camera poses via C FFI

## Usage

```bash
# 1. Build the sim_bridge Rust library
cd sim_bridge && cargo build --release

# 2. Launch UE5
open -a "UnrealEditor" BehaveSim.uproject

# 3. Press Play in UE5, then run the robot stack:
cargo run --bin runmode -- robot/modes/metaverse.yaml
```

## Troubleshooting: UE5 module load failure

If UE5 shows **"The game module 'BehaveSim' could not be loaded"**, the compiled module binaries are stale or incompatible. Clean and rebuild:

```bash
# From this directory (robot/nodes/sim_metaverse/):

# 1. Remove all UE5 generated/cached directories
rm -rf Binaries/ DerivedDataCache/ Intermediate/ Saved/

# 2. Clean the sim_bridge Rust build
cd sim_bridge && cargo clean && cd ..

# 3. Rebuild the sim_bridge
cd sim_bridge && cargo build --release && cd ..

# 4. Regenerate UE5 project files (macOS)
/Users/Shared/Epic\ Games/UE_5.*/Engine/Build/BatchFiles/Mac/GenerateProjectFiles.sh \
    "$(pwd)/BehaveSim.uproject" -game

# 5. Reopen in UE5
open -a "UnrealEditor" BehaveSim.uproject
```

If step 4 fails (path depends on your UE5 install), you can also just open the `.uproject` directly -- UE5 will rebuild the module on first launch.

## Topics

- Subscribes to: `behave/SimRequest`
- Publishes to: `behave/SimStatus`
