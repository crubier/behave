# Node: Control Sim

Simulated flight controller. Receives ControlCommand messages and tracks simulated drone state (position, altitude, armed status, flight mode). Publishes ControlAck after each command and DroneState telemetry at 10 Hz.

## Topics

- Subscribes to: `behave/ControlCommand`
- Publishes to: `behave/ControlAck`
