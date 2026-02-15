# from_proto/

Renders a protobuf `ActionNode` tree as Dioxus elements.

Bridge between the wire format (protobuf from GCS) and the Dioxus renderer. Recursively maps each `ActionArgs` variant to the matching core element.

## Files

- `mod.rs` -- `RenderActionNode` Dioxus component.
