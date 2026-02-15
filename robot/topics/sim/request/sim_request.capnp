@0xa8d3e5f1b2c4d6e8;

# ── Sim Request ────────────────────────────────────────────────
#
# Commands sent to the UE5 simulator to position the virtual camera.

struct CameraPose {
  # Position in meters (X = forward, Y = right, Z = up)
  x @0 :Float64;
  y @1 :Float64;
  z @2 :Float64;

  # Orientation as unit quaternion
  qw @3 :Float64;
  qx @4 :Float64;
  qy @5 :Float64;
  qz @6 :Float64;
}

struct SimRequest {
  pose        @0 :CameraPose;   # Requested camera pose
  utime       @1 :UInt64;       # Monotonic timestamp in microseconds
}
