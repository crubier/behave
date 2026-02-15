@0x8d1079e8989b5915;

# ── Control Status ─────────────────────────────────────────────
#
# Published by Control nodes. Flight controller state only.
# Position is in SenseStatus.

enum FlightMode {
  idle      @0;
  takingOff @1;
  flying    @2;
  landing   @3;
  hovering  @4;
  returning @5;
}

struct ControlStatus {
  armed      @0 :Bool;
  mode       @1 :FlightMode;
  batteryPct @2 :Float64;
}
