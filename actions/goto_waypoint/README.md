# Action: GotoWaypoint

Leaf action that flies the vehicle to a specific local-frame coordinate at a given altitude and speed.

Sends a goto ControlRequest, then simulates reducing remaining distance by 200m per tick.

## Args

- `eastingM` -- local-frame easting in metres
- `northingM` -- local-frame northing in metres
- `altitudeM` -- altitude in metres (AGL)
- `speedMs` -- cruise speed in m/s (0 = default)
