# land/

Land leaf action. Sends a land command and monitors altitude until touchdown.

## Args

- `descent_speed_ms` -- descent speed in m/s

## Files

- `land.proto` -- LandArgs, LandResult, LandState, LandInput, LandOutput
- `mod.rs` -- `start()` sends land command, `tick()` polls altitude
- `index.tsx` -- React UI component
