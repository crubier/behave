# Node: Behave

Tick-based behavior tree executor. Receives Mission messages containing an ActionArgs behavior tree, builds the runtime tree, and ticks it every 100ms until completion.

Fully action-agnostic -- delegates all action-specific logic to the `actions/` modules.

## Topics

- Subscribes to: `behave/Mission`
- Publishes to: `behave/ControlCommand`
