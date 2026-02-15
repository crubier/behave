@0xef13fae48f05fdaa;

struct GotoWaypoint {
  latitudeDeg  @0 :Float64;  # WGS-84 latitude  in degrees
  longitudeDeg @1 :Float64;  # WGS-84 longitude in degrees
  altitudeM    @2 :Float64;  # Altitude in metres (AGL)
  speedMs      @3 :Float64;  # Cruise speed in m/s (0 = default)
}
