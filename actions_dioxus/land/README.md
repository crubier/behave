# land/

Land leaf action. Sends a land command and monitors altitude until touchdown.

## Files

- `land.proto` -- `LandArgs` (descent_speed_ms), `LandOutput` (current altitude), `LandResult` (final altitude), `LandInput`, `LandState`.
- `mod.rs` -- `LandNode` type alias + `Behavior` impl + `Land` Dioxus component.
