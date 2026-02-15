@0x85716df07291b1d0;

# ── Takeoff ─────────────────────────────────────────────────────

struct TakeoffArgs {
  altitudeM @0 :Float64;   # Target altitude in metres (AGL)
}

struct TakeoffResult {
  reachedAltitudeM @0 :Float64;  # Altitude actually reached
  success          @1 :Bool;
}

enum TakeoffPhase {
  idle     @0;
  climbing @1;
  reached  @2;
}

struct TakeoffState {
  currentAltitudeM @0 :Float64;
  phase            @1 :TakeoffPhase;
}

struct TakeoffInput {
  # Real-time signals from GCS (reserved for future use)
}

struct TakeoffOutput {
  progressPct @0 :Float64;  # 0..100
}
