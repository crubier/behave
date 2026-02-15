@0xc7bbdceaefc35aee;

# ── TakePhoto ───────────────────────────────────────────────────

struct TakePhotoArgs {
  tag @0 :Text;  # Optional label / tag for the captured image
}

struct TakePhotoResult {
  success  @0 :Bool;
  filePath @1 :Text;   # Path to the captured image (if any)
}

enum TakePhotoPhase {
  idle       @0;
  capturing  @1;
  done       @2;
}

struct TakePhotoState {
  phase @0 :TakePhotoPhase;
}

struct TakePhotoInput {
  # Real-time signals from GCS (reserved for future use)
}

struct TakePhotoOutput {
  captured @0 :Bool;  # Whether the photo has been taken
}
