# actions_dioxus/

Behavior tree execution engine using Dioxus as a custom renderer.

## Structure

```
actions_dioxus/
├── core/               Core framework (traits, renderer, data types)
├── sequence/           Sequence node (all children must succeed)
├── fallback/           Fallback node (first child to succeed wins)
├── parallel/           Parallel node (all children run at once)
├── takeoff/            Takeoff leaf action
├── land/               Land leaf action
├── goto_waypoint/      Goto waypoint leaf action
├── home/               Return-home leaf action
├── take_photo/         Take photo leaf action
├── from_proto/         Renders a protobuf ActionNode tree as Dioxus elements
├── example_mission/    Example missions built from components
├── action.proto        Top-level composed protobuf types (ActionArgs, ActionNode, Run)
└── mod.rs              Module root and re-exports
```

## Files

- `action.proto` -- Composed oneof types (`ActionArgs`, `ActionOutput`, `ActionResult`, `ActionInput`, `ActionState`), the `ActionNode` tree message, and the `Run` execution record.
- `mod.rs` -- Declares all submodules and includes the generated `action_proto` code.

## How it works

Each node type lives in its own folder with a `.proto` (schema) and `mod.rs` (behavior + component). The `core/` framework provides the generic `ActionNode<A,O,R,S>` struct and the `Behavior` trait. You define a type alias and implement `Behavior` -- everything else (struct fields, constructors, encoding) is automatic.

Missions are composed using Dioxus components (PascalCase wrappers like `Takeoff`, `Goto`, `Sequence`). The `BehaviorTreeRenderer` executes the element tree.
