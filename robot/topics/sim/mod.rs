use iceoryx2::prelude::*;

/// Camera pose (position + orientation), shared by SimRequest and SimStatus.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, ZeroCopySend)]
pub struct CameraPose {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub qw: f64,
    pub qx: f64,
    pub qy: f64,
    pub qz: f64,
}

pub mod request;
pub mod status;
