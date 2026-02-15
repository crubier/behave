# take_photo/

Take photo leaf action. Triggers the camera and waits for acknowledgment.

## Args

None.

## Files

- `take_photo.proto` -- TakePhotoArgs, TakePhotoResult, TakePhotoState, TakePhotoInput, TakePhotoOutput (captured)
- `mod.rs` -- `start()` triggers camera, `tick()` waits for ack
- `index.tsx` -- React UI component
