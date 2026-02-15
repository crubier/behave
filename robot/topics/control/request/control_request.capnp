@0xdb4cc1c9144591bb;

# ── Control Request ────────────────────────────────────────────
#
# Flight commands sent from Behave -> Control.

struct ControlRequest {
  id @0 :UInt64;  # Unique command identifier for ack correlation

  union {
    arm          @1  :Void;
    disarm       @2  :Void;
    takeoff      @3  :TakeoffCmd;
    land         @4  :LandCmd;
    hover        @5  :Void;
    returnHome   @6  :ReturnHomeCmd;
    goto         @7  :GotoCmd;
    triggerCamera @8 :CameraCmd;
  }
}

struct TakeoffCmd {
  altitudeM @0 :Float64;
}

struct LandCmd {
  descentSpeedMs @0 :Float64;  # 0 = default
}

struct ReturnHomeCmd {
  altitudeM @0 :Float64;
}

struct GotoCmd {
  eastingM   @0 :Float64;
  northingM  @1 :Float64;
  altitudeM  @2 :Float64;
  speedMs    @3 :Float64;   # 0 = default
}

struct CameraCmd {
  tag @0 :Text;
}
