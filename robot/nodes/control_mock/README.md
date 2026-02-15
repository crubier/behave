# Node: Control Mock

Dumb mock control node for lightweight testing. Receives ControlRequest messages, logs each one as a single line, and immediately sends back a successful ControlResponse. No state tracking, no simulation.

## Topics

- Subscribes to: `behave/ControlRequest`
- Publishes to: `behave/ControlResponse`
