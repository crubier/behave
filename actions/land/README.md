# Action: Land

Leaf action that commands the vehicle to land at its current position.

Sends a land ControlCommand, then simulates descending by 10m per tick until touchdown.

## Args

- `descentSpeedMs` -- vertical descent speed in m/s (0 = default)
