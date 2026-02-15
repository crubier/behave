# Action: GotoWaypoint

Leaf action that flies the vehicle to a specific WGS-84 coordinate at a given altitude and speed.

Sends a goto ControlCommand, then simulates reducing remaining distance by 200m per tick.

## Args

- `latitudeDeg` -- WGS-84 latitude in degrees
- `longitudeDeg` -- WGS-84 longitude in degrees
- `altitudeM` -- altitude in metres (AGL)
- `speedMs` -- cruise speed in m/s (0 = default)
