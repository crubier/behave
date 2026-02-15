@0xf62e4cfda840ebe9;

# ── Sim Status ─────────────────────────────────────────────────
#
# Telemetry from the simulator back to the robot.

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

struct SimStatus {
  pose        @0 :CameraPose;   # Real camera pose from simulator
  utime       @1 :UInt64;       # Simulator timestamp in microseconds
}
