# Node: Sense Mock

Mock sense node for lightweight testing. Subscribes to `behave/SimStatus` and forwards the pose as `behave/SenseStatus` with stub vehicle state.

## Topics

- Subscribes to: `behave/SimStatus`
- Publishes to: `behave/SenseStatus`
