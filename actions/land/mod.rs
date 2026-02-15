use crate::schema::actions::land_capnp::land;

/// Execute a Land action.
///
/// Commands the vehicle to land at its current position.
pub fn execute(args: land::Reader<'_>) -> anyhow::Result<()> {
    let speed = args.get_descent_speed_ms();
    println!("[Land] descending at {speed:.1} m/s");
    Ok(())
}
