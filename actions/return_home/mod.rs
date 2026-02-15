use crate::schema::actions::return_home_capnp::return_home;

/// Execute a ReturnHome action.
///
/// Commands the vehicle to return to its home position at a given altitude.
pub fn execute(args: return_home::Reader<'_>) -> anyhow::Result<()> {
    let altitude = args.get_altitude_m();
    println!("[ReturnHome] returning home at {altitude:.1} m AGL");
    Ok(())
}
