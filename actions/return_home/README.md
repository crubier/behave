# return_home/

Return home leaf action. Sends a return-home command and monitors flight mode until arrival.

## Args

- `altitude_m` -- return altitude

## Files

- `return_home.proto` -- ReturnHomeArgs, ReturnHomeResult, ReturnHomeState, ReturnHomeInput, ReturnHomeOutput
- `mod.rs` -- `start()` sends RTH command, `tick()` polls mode
- `index.tsx` -- React UI component
