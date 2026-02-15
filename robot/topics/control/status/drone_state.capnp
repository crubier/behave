@0x8d1079e8989b5915;

# ── Drone State Telemetry ───────────────────────────────────────
#
# Published by Control for all nodes to consume.

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
