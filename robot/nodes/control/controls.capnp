@0xef3b377af0cfec9f;

# ── Drone Controls API ──────────────────────────────────────────
#
# Commands sent from Behave -> Control to fly the drone.
# Acknowledgments and telemetry flow back from Control.

struct ControlCommand {
  id @0 :UInt64;  # Unique command identifier for ack correlation

  union {
    # Flight mode
    arm          @1  :Void;
    disarm       @2  :Void;
    takeoff      @3  :TakeoffCmd;
    land         @4  :LandCmd;
    hover        @5  :Void;
    returnHome   @6  :ReturnHomeCmd;

    # Navigation
    goto         @7  :GotoCmd;

    # Payload
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
  latitudeDeg  @0 :Float64;
  longitudeDeg @1 :Float64;
  altitudeM    @2 :Float64;
  speedMs      @3 :Float64;   # 0 = default
}

struct CameraCmd {
  tag @0 :Text;
}

# ── Acknowledgment ──────────────────────────────────────────────

struct ControlAck {
  commandId @0 :UInt64;
  success   @1 :Bool;
  message   @2 :Text;
}

# ── Drone telemetry ─────────────────────────────────────────────

enum FlightMode {
  idle      @0;
  takingOff @1;
  flying    @2;
  landing   @3;
  hovering  @4;
  returning @5;
}

struct DroneState {
  latitudeDeg  @0 :Float64;
  longitudeDeg @1 :Float64;
  altitudeM    @2 :Float64;
  armed        @3 :Bool;
  mode         @4 :FlightMode;
  batteryPct   @5 :Float64;
}
