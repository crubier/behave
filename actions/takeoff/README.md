# Action: Takeoff

Leaf action that commands the vehicle to take off to a target altitude (AGL).

Sends a takeoff ControlCommand, then simulates climbing at 5m per tick until the target altitude is reached.

## Args

- `altitudeM` -- target altitude in metres above ground level
