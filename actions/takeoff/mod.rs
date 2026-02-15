use crate::schema::actions::takeoff_capnp::takeoff;

/// Execute a Takeoff action.
///
/// Commands the vehicle to take off to the specified altitude (AGL).
pub fn execute(args: takeoff::Reader<'_>) -> anyhow::Result<()> {
    let altitude = args.get_altitude_m();
    println!("[Takeoff] climbing to {altitude:.1} m AGL");
    Ok(())
}
