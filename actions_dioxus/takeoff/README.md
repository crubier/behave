# takeoff/

Takeoff leaf action. Sends a takeoff command and monitors altitude until the target is reached.

## Files

- `takeoff.proto` -- `TakeoffArgs` (altitude_m), `TakeoffOutput` (progress), `TakeoffResult` (reached altitude), `TakeoffInput`, `TakeoffState`.
- `mod.rs` -- `TakeoffNode` type alias + `Behavior` impl + `Takeoff` Dioxus component.
