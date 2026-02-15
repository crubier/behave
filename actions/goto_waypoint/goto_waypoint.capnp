@0xef13fae48f05fdaa;

# ── GotoWaypoint ────────────────────────────────────────────────

struct GotoWaypointArgs {
  eastingM   @0 :Float64;  # Local-frame easting in metres
  northingM  @1 :Float64;  # Local-frame northing in metres
  altitudeM  @2 :Float64;  # Altitude in metres (AGL)
  speedMs    @3 :Float64;  # Cruise speed in m/s (0 = default)
}

struct GotoWaypointResult {
  finalEastingM   @0 :Float64;
  finalNorthingM  @1 :Float64;
  finalAltitudeM  @2 :Float64;
  success         @3 :Bool;
}

enum GotoWaypointPhase {
  idle       @0;
  enRoute    @1;
  arrived    @2;
}

struct GotoWaypointState {
  currentEastingM     @0 :Float64;
  currentNorthingM    @1 :Float64;
  currentAltitudeM    @2 :Float64;
  remainingDistanceM  @3 :Float64;
  phase               @4 :GotoWaypointPhase;
}

struct GotoWaypointInput {
  # Real-time signals from GCS (reserved for future use)
}

struct GotoWaypointOutput {
  progressPct        @0 :Float64;  # 0..100
  etaSeconds         @1 :Float64;
  remainingDistanceM @2 :Float64;
}
