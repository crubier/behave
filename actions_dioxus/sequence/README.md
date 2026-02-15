# sequence/

Sequence composite node. Activates children left-to-right. Succeeds if all succeed, fails on first failure.

## Files

- `sequence.proto` -- `SequenceArgs`, `SequenceOutput`, `SequenceResult`, `SequenceInput`, `SequenceState`.
- `mod.rs` -- `SequenceNode` type alias + `Behavior` impl + `Sequence` Dioxus component.
