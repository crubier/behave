# goto_waypoint/

Goto waypoint leaf action. Sends a goto command and monitors position until arrival.

## Files

- `goto.proto` -- `GotoArgs` (easting, northing, altitude, speed), `GotoOutput` (target/current position, remaining distance), `GotoResult` (final position), `GotoInput`, `GotoState`.
- `mod.rs` -- `GotoWaypointNode` type alias + `Behavior` impl + `Goto` Dioxus component.
