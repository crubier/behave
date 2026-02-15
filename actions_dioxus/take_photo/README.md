# take_photo/

Take photo leaf action. Triggers the camera and waits for acknowledgment.

## Files

- `photo.proto` -- `PhotoArgs`, `PhotoOutput` (acked), `PhotoResult` (captured), `PhotoInput`, `PhotoState`.
- `mod.rs` -- `TakePhotoNode` type alias + `Behavior` impl + `Photo` Dioxus component.
