# core/

Framework for the behavior tree renderer. Node authors don't edit these files.

## Files

- `behavior.rs` -- `ActionNode<A,O,R,S>` generic struct, `Behavior` trait (what nodes implement), `NodeBehavior` trait (what the renderer uses), and the blanket impl that bridges them.
- `data.rs` -- `ProtoBytes` (passes serialized proto through Dioxus attributes), `Run` (execution record with timing, status, and protobuf data), time helpers.
- `elements.rs` -- Custom Dioxus element definitions (`takeoff`, `land`, `goto`, etc.) for the `rsx!` macro.
- `node.rs` -- `RendererNode` (the renderer's internal node, stores tag, args bytes, parent/children, state, and run record).
- `renderer.rs` -- `BehaviorTreeRenderer` implementing `WriteMutations`. Manages the node tree, activation/deactivation, tick loop, result propagation, run tracking, and commands.
- `mod.rs` -- Re-exports.
