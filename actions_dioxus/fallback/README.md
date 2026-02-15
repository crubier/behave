# fallback/

Fallback composite node. Activates children left-to-right. Succeeds on first success, fails if all fail.

## Files

- `fallback.proto` -- `FallbackArgs`, `FallbackOutput`, `FallbackResult`, `FallbackInput`, `FallbackState`.
- `mod.rs` -- `FallbackNode` type alias + `Behavior` impl + `Fallback` Dioxus component.
