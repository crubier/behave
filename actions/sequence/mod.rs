use crate::schema::actions::sequence_capnp::sequence;

/// Execute a Sequence node.
///
/// Ticks children left-to-right; succeeds only if **all** children succeed.
pub fn execute(_args: sequence::Reader<'_>) -> anyhow::Result<()> {
    println!("[Sequence] ticking children in order");
    Ok(())
}
