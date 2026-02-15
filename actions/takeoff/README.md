# takeoff/

Takeoff leaf action. Sends a takeoff command and monitors altitude until the target is reached.

## Args

- `altitude_m` -- target altitude in meters

## Files

- `takeoff.proto` -- TakeoffArgs, TakeoffResult (reached_altitude_m), TakeoffState, TakeoffInput, TakeoffOutput (progress_pct)
- `mod.rs` -- `start()` sends takeoff command, `tick()` polls altitude
- `index.tsx` -- React UI component
