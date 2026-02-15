/// TakePhoto leaf action.
///
/// Captures a photo, optionally tagged with a label.
pub struct TakePhoto {
    pub tag: Option<String>,
}
