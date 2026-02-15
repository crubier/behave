# Node: Sim Metaverse

UE5 metaverse camera simulator. Subscribes to `behave/SimRequest`, simulates simple physics by linearly interpolating position and quaternion at fixed speeds, and publishes `behave/SimStatus` at 60 Hz.

Also sends the current pose to UE5 over UDP (port 9876) as a flat 64-byte packet each tick. UE5 receives it directly with a `FUdpSocketReceiver` -- no Rust bridge needed.

## Components

- **mod.rs / main.rs** -- iceoryx2 sim node (physics interpolation at 60 Hz + UDP sender)
- **BehaveSim.uproject** -- UE5 project with a camera pawn driven by UDP
- **Source/** -- UE5 C++ source (SimCameraPawn receives UDP, applies pose)

## Usage

```bash
# 1. Launch UE5
open -a "UnrealEditor" BehaveSim.uproject

# 2. Press Play in UE5, then run the robot stack:
cargo run --bin runmode -- robot/modes/metaverse.yaml
```

No separate bridge build step needed -- the sim_metaverse node sends UDP directly.

## UDP Protocol

Port `9876`, localhost. 64-byte little-endian packet:

| Offset | Type   | Field  |
|--------|--------|--------|
| 0      | f64    | x      |
| 8      | f64    | y      |
| 16     | f64    | z      |
| 24     | f64    | qw     |
| 32     | f64    | qx     |
| 40     | f64    | qy     |
| 48     | f64    | qz     |
| 56     | u64    | utime  |

Position is in meters, orientation is a unit quaternion, utime is microseconds since epoch.

## Troubleshooting: UE5 module load failure

If UE5 shows "The game module 'BehaveSim' could not be loaded", clean and reopen:

```bash
rm -rf Binaries/ DerivedDataCache/ Intermediate/ Saved/
open -a "UnrealEditor" BehaveSim.uproject
```

## Topics

- Subscribes to: `behave/SimRequest`
- Publishes to: `behave/SimStatus`
- Sends UDP to: `127.0.0.1:9876` (UE5)
