@0xef13fae48f05fdaa;

# ── GotoWaypoint ────────────────────────────────────────────────

struct GotoWaypointArgs {
  latitudeDeg  @0 :Float64;  # WGS-84 latitude  in degrees
  longitudeDeg @1 :Float64;  # WGS-84 longitude in degrees
  altitudeM    @2 :Float64;  # Altitude in metres (AGL)
  speedMs      @3 :Float64;  # Cruise speed in m/s (0 = default)
}

struct GotoWaypointResult {
  finalLatitudeDeg  @0 :Float64;
  finalLongitudeDeg @1 :Float64;
  finalAltitudeM    @2 :Float64;
  success           @3 :Bool;
}

enum GotoWaypointPhase {
  idle       @0;
  enRoute    @1;
  arrived    @2;
}

struct GotoWaypointState {
  currentLatitudeDeg  @0 :Float64;
  currentLongitudeDeg @1 :Float64;
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
