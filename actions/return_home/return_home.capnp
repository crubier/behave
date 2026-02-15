@0x90cbbba6658c0442;

# ── ReturnHome ──────────────────────────────────────────────────

struct ReturnHomeArgs {
  altitudeM @0 :Float64;   # Altitude to maintain during return (AGL)
}

struct ReturnHomeResult {
  success @0 :Bool;
}

enum ReturnHomePhase {
  idle      @0;
  climbing  @1;
  enRoute   @2;
  arrived   @3;
}

struct ReturnHomeState {
  remainingDistanceM @0 :Float64;
  phase              @1 :ReturnHomePhase;
}

struct ReturnHomeInput {
  # Real-time signals from GCS (reserved for future use)
}

struct ReturnHomeOutput {
  progressPct @0 :Float64;  # 0..100
  etaSeconds  @1 :Float64;
}
