use crate::schema::actions::take_photo_capnp::take_photo;

/// Execute a TakePhoto action.
///
/// Captures a photo, optionally tagged with a label.
pub fn execute(args: take_photo::Reader<'_>) -> anyhow::Result<()> {
    let tag = args.get_tag()?.to_str()?;
    if tag.is_empty() {
        println!("[TakePhoto] capturing photo");
    } else {
        println!("[TakePhoto] capturing photo (tag: {tag})");
    }
    Ok(())
}
