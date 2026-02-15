# actions/

Behavior tree action definitions. Each action type lives in its own folder with colocated schema, Rust behavior, and React UI.

## Structure

```
actions/
├── action.proto          Top-level composed types (ActionArgs, ActionResult, ActionNode, ActionRun)
├── mod.rs                Runtime: ActionNode, dispatch, tree parsing
├── io.rs                 ActionIO interface (what actions can see/do)
│
├── sequence/             Sequence: all children must succeed (left-to-right)
├── fallback/             Fallback: first child to succeed wins (left-to-right)
├── parallel/             Parallel: all children run at once, all must succeed
├── concurrent/           Concurrent: all children run at once, configurable success threshold
├── loop/                 Loop: repeats its child N times (or infinitely)
│
├── takeoff/              Takeoff: climb to target altitude
├── land/                 Land: descend to touchdown
├── goto_waypoint/        Goto: fly to a position
├── return_home/          Return home: fly back to launch point
└── take_photo/           Photo: trigger camera and wait for ack
```

## Per-action folder

Each folder contains:

- `*.proto` -- Protobuf schema defining 5 message types: Args, Result, State, Input, Output
- `mod.rs` -- Rust runtime: `start()` and `tick()` functions, `pub mod proto` with generated types
- `index.tsx` -- React UI component (for the GCS web interface)

## Key types

- **ActionNode** -- tree structure sent from GCS (id + args + children)
- **ActionRun** -- execution record (run_id, node_id, timing, status, output/result/state, child runs)
- **ActionArgs/Result/State/Input/Output** -- oneof unions over all action-specific types

## Adding a new action

1. Create `actions/my_action/my_action.proto` with the 5 message types
2. Create `actions/my_action/mod.rs` with `start()`, `tick()`, and `pub mod proto`
3. Add it to `action.proto` (all 5 oneof unions)
4. Add it to `actions/mod.rs` (module, enums, dispatch)
5. Add extern_path mapping in `build.rs`
