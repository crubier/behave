# Node: Control Mock

Dumb mock control node for lightweight testing. Receives ControlCommand messages, logs each one as a single line, and immediately sends back a successful ControlAck. No state tracking, no simulation.

## Topics

- Subscribes to: `behave/ControlCommand`
- Publishes to: `behave/ControlAck`
