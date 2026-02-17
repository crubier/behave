# Node: Sim Metaverse

UE5 metaverse camera simulator. Subscribes to `behave/SimRequest`, simulates simple physics by linearly interpolating position and quaternion at fixed speeds, and publishes `behave/SimStatus` at 60 Hz.

UE5 reads `SimStatus` directly from iceoryx2 shared memory -- zero-copy, no network.

## Components

- **mod.rs / main.rs** -- iceoryx2 sim node (physics interpolation at 60 Hz)
- **BehaveSim.uproject** -- UE5 project with a camera pawn driven by iceoryx2
- **Source/BehaveSim/** -- UE5 C++ source (SimCameraPawn subscribes to SimStatus via iceoryx2)
- **Source/ThirdParty/iceoryx2_bridge/** -- thin C wrapper around iceoryx2 C++ API, built as a shared library so UE5 can link it without header conflicts

## Building

Prerequisites: Rust toolchain, CMake 3.22+, Xcode command-line tools, Unreal Engine 5.7.

```bash
# Build the iceoryx2 bridge + UE5 module (one command)
./robot/nodes/sim_metaverse/build_ue5.sh
```

Or step by step:

```bash
cd robot/nodes/sim_metaverse/Source/ThirdParty/iceoryx2_bridge

# 1. Configure and build (fetches iceoryx2 v0.7.0 via FetchContent)
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build -j$(sysctl -n hw.ncpu)

# 2. Install bridge dylib + header to ../iceoryx2/ for UE5
cmake --install build --prefix build/_install

# 3. Create version symlinks
cd ../iceoryx2/lib
ln -sf libiceoryx_hoofs.2.95.7.dylib libiceoryx_hoofs.2.dylib
ln -sf libiceoryx_platform.2.95.7.dylib libiceoryx_platform.2.dylib
```

## Usage

```bash
# Run the robot stack -- UE5 launches automatically in standalone game mode
cargo run --bin runmode -- robot/modes/metaverse.yaml
```

The `sim-metaverse` node manages UE5 startup based on the `ue_mode` parameter:

| `ue_mode`  | Behavior                                                       |
|------------|----------------------------------------------------------------|
| `game`     | Auto-launches UE5 in standalone game mode (`-game`), no editor |
| `editor`   | Logs the `open` command for the user to launch UE5 manually    |

Configuration (in `metaverse.yaml`):

| YAML param   | Env var            | Default | Description                         |
|--------------|--------------------|---------|-------------------------------------|
| `ue_project` | `BEHAVE_UE_PROJECT`| *(none)*| Path to `.uproject` (required)      |
| `ue_mode`    | `BEHAVE_UE_MODE`   | `game`  | `game` or `editor`                  |
| `ue_res_x`   | `BEHAVE_UE_RES_X`  | 1920    | Horizontal resolution (pixels)      |
| `ue_res_y`   | `BEHAVE_UE_RES_Y`  | 1080    | Vertical resolution (pixels)        |
| `ue_fps`     | `BEHAVE_UE_FPS`    | 30      | Max FPS cap                         |

Resolution is automatically adjusted for Retina displays.

## Architecture

```
sim_metaverse (Rust)                    UE5 (C++)
┌─────────────────────┐                ┌─────────────────────────┐
│ subscribe SimRequest │                │ SimCameraPawn           │
│ physics interpolation│                │   ↓                     │
│ publish SimStatus ───┼── iceoryx2 ──→│ IoxBridge (C wrapper)   │
│                      │  shared mem   │   ↓                     │
│ auto-launch UE5      │               │ SetActorLocation/Rot    │
└─────────────────────┘                └─────────────────────────┘
```

The iceoryx2 bridge uses `IOX2_TYPE_NAME` for cross-language type matching between Rust and C++.

## Topics

- Subscribes to: `behave/SimRequest`
- Publishes to: `behave/SimStatus` (consumed by UE5 + sense node)

## Troubleshooting

**UE5 "BehaveSim could not be compiled"** -- rebuild the iceoryx2 bridge first:

```bash
./build_ue5.sh
```

**UE5 "The game module 'BehaveSim' could not be loaded"** -- clean and rebuild:

```bash
rm -rf Binaries/ Intermediate/
./build_ue5.sh
```
