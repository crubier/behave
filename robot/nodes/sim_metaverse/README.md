# Node: Sim Metaverse

UE5 metaverse camera simulator. Subscribes to `behave/SimRequest`, simulates simple physics by linearly interpolating position and quaternion at fixed speeds, and publishes `behave/SimStatus` at 60 Hz.

Also sends the current pose to UE5 over UDP (port 9876) as a flat 64-byte packet each tick. UE5 receives it directly with a `FUdpSocketReceiver` -- no Rust bridge needed.

## Components

- **mod.rs / main.rs** -- iceoryx2 sim node (physics interpolation at 60 Hz + UDP sender)
- **BehaveSim.uproject** -- UE5 project with a camera pawn driven by UDP
- **Source/** -- UE5 C++ source (SimCameraPawn receives UDP, applies pose)

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

Resolution and FPS are configurable (apply to `game` mode):

| YAML param   | Env var            | Default | Description                    |
|--------------|--------------------|---------|--------------------------------|
| `ue_project` | `BEHAVE_UE_PROJECT`| *(none)*| Path to `.uproject` (required) |
| `ue_mode`    | `BEHAVE_UE_MODE`   | `game`  | `game` or `editor`             |
| `ue_res_x`   | `BEHAVE_UE_RES_X`  | 1920    | Horizontal resolution          |
| `ue_res_y`   | `BEHAVE_UE_RES_Y`  | 1080    | Vertical resolution            |
| `ue_fps`     | `BEHAVE_UE_FPS`    | 30      | Max FPS cap                    |

In `editor` mode, open UE5 manually and press Play before running the stack:

```bash
open -a "UnrealEditor" robot/nodes/sim_metaverse/BehaveSim.uproject
# Press Play in UE5, then run:
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
