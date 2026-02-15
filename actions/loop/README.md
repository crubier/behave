# loop/

Loop composite. Repeats its single child N times. Fails if any iteration fails.

- `max_iterations = 0` -- infinite loop
- `max_iterations = N` -- loop N times then succeed

## Files

- `loop.proto` -- LoopArgs (max_iterations), LoopResult, LoopState, LoopInput, LoopOutput
- `mod.rs` -- `start()` / `tick()` runtime
