use crate::schema::actions::goto_waypoint_capnp::goto_waypoint;

/// Execute a GotoWaypoint action.
///
/// Flies the vehicle to a WGS-84 coordinate at a given altitude and speed.
pub fn execute(args: goto_waypoint::Reader<'_>) -> anyhow::Result<()> {
    let lat = args.get_latitude_deg();
    let lon = args.get_longitude_deg();
    let alt = args.get_altitude_m();
    let spd = args.get_speed_ms();
    println!("[GotoWaypoint] flying to ({lat:.6}, {lon:.6}) at {alt:.1} m, {spd:.1} m/s");
    Ok(())
}
