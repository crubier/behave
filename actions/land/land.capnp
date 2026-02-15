@0xe85e5a745e506cd2;

# ── Land ────────────────────────────────────────────────────────

struct LandArgs {
  descentSpeedMs @0 :Float64;  # Vertical descent speed in m/s (0 = default)
}

struct LandResult {
  success @0 :Bool;
}

enum LandPhase {
  idle       @0;
  descending @1;
  touchdown  @2;
}

struct LandState {
  currentAltitudeM @0 :Float64;
  phase            @1 :LandPhase;
}

struct LandInput {
  # Real-time signals from GCS (reserved for future use)
}

struct LandOutput {
  progressPct @0 :Float64;  # 0..100
}
