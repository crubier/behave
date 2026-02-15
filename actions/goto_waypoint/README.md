# goto_waypoint/

Goto waypoint leaf action. Sends a goto command and monitors position until arrival.

## Args

- `easting_m`, `northing_m` -- target position
- `altitude_m` -- target altitude
- `speed_ms` -- flight speed

## Files

- `goto_waypoint.proto` -- GotoWaypointArgs, GotoWaypointResult (final position), GotoWaypointState, GotoWaypointInput, GotoWaypointOutput (remaining_distance_m)
- `mod.rs` -- `start()` sends goto command, `tick()` polls position
- `index.tsx` -- React UI component
