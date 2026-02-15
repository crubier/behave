/// GotoWaypoint leaf action.
///
/// Flies the vehicle to a specific WGS-84 coordinate at a given altitude and speed.
pub struct GotoWaypoint {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub altitude_m: f64,
    pub speed_ms: f64,
}
