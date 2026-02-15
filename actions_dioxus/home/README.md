# home/

Return-home leaf action. Sends a return-home command and monitors flight mode until arrival.

## Files

- `home.proto` -- `HomeArgs` (altitude_m), `HomeOutput` (mode), `HomeResult` (final mode), `HomeInput`, `HomeState`.
- `mod.rs` -- `HomeNode` type alias + `Behavior` impl + `Home` Dioxus component.
