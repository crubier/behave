# fallback/

Fallback composite. Ticks children left-to-right. Succeeds on first success, fails if all fail.

## Files

- `fallback.proto` -- FallbackArgs, FallbackResult, FallbackState, FallbackInput, FallbackOutput
- `mod.rs` -- `start()` / `tick()` runtime
- `index.tsx` -- React UI component
