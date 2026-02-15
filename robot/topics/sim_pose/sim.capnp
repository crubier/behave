@0xa8d3e5f1b2c4d6e8;

# ── Simulator Camera Pose ──────────────────────────────────────
#
# Commands sent to the UE5 simulator to position the virtual camera.
# Coordinates use meters with the convention:
#   X = forward, Y = right, Z = up
# Orientation is a unit quaternion (w, x, y, z).

struct CameraPose {
  # Position in meters
  x @0 :Float64;
  y @1 :Float64;
  z @2 :Float64;

  # Orientation as unit quaternion
  qw @3 :Float64;
  qx @4 :Float64;
  qy @5 :Float64;
  qz @6 :Float64;

  # Monotonic timestamp in microseconds
  timestampUs @7 :UInt64;
}
