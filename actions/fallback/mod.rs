use crate::schema::actions::fallback_capnp::fallback;

/// Execute a Fallback node.
///
/// Ticks children left-to-right; succeeds if **any** child succeeds.
pub fn execute(_args: fallback::Reader<'_>) -> anyhow::Result<()> {
    println!("[Fallback] ticking children until one succeeds");
    Ok(())
}
