# Node: Control Metaverse

Flight controller for the UE5 metaverse simulation. Receives ControlRequest messages and tracks simulated drone state (position, altitude, armed status, flight mode). Publishes ControlResponse after each command and ControlStatus telemetry at 10 Hz.

## Topics

- Subscribes to: `behave/ControlRequest`
- Publishes to: `behave/ControlResponse`, `behave/ControlStatus`
