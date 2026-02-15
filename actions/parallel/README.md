# parallel/

Parallel composite. Activates all children at once. Succeeds when all succeed, fails on first failure.

## Files

- `parallel.proto` -- ParallelArgs, ParallelResult, ParallelState, ParallelInput, ParallelOutput
- `mod.rs` -- `start()` / `tick()` runtime
