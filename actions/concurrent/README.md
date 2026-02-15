# concurrent/

Concurrent composite. Activates all children at once. Succeeds when `success_threshold` children succeed. Fails when threshold becomes unreachable.

- `success_threshold = 0` -- all must succeed (same as parallel)
- `success_threshold = 1` -- any one succeeding is enough (race)
- `success_threshold = N` -- at least N must succeed

## Files

- `concurrent.proto` -- ConcurrentArgs (success_threshold), ConcurrentResult, ConcurrentState, ConcurrentInput, ConcurrentOutput
- `mod.rs` -- `start()` / `tick()` runtime
