# parallel/

Parallel composite node. Activates all children simultaneously. Succeeds when all succeed, fails on first failure.

## Files

- `parallel.proto` -- `ParallelArgs`, `ParallelOutput`, `ParallelResult`, `ParallelInput`, `ParallelState`.
- `mod.rs` -- `ParallelNode` type alias + `Behavior` impl + `Parallel` Dioxus component.
