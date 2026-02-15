# Node: Sim Mock

Instant camera pose simulator for lightweight testing. Subscribes to `behave/SimRequest` and immediately publishes a `behave/SimStatus` with the requested pose as the current pose. No physics, no delay.

## Topics

- Subscribes to: `behave/SimRequest`
- Publishes to: `behave/SimStatus`
