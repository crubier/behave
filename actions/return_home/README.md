# Action: ReturnHome

Leaf action that commands the vehicle to return to its home position at a specified altitude.

Sends a return-home ControlRequest, then simulates reducing remaining distance by 200m per tick.

## Args

- `altitudeM` -- altitude to maintain during return (AGL)
