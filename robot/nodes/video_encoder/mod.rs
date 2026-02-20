//! Video Encoder node -- H.264 encoding pipeline.
//!
//! Subscribes to raw BGRA camera frames on `behave/video/CameraRawFrame`,
//! encodes them to H.264 using OpenH264, and publishes the NAL units
//! on `behave/video/CameraEncodedFrame`.

use anyhow::Result;
use iceoryx2::prelude::*;
use log::info;
use openh264::encoder::Encoder;
use openh264::formats::{RgbSliceU8, YUVBuffer};

use behave::topics;
use behave::topics::video::camera_raw_frame::{self, CameraRawFrame, WIDTH, HEIGHT};
use behave::topics::video::camera_encoded_frame;

pub fn run() -> Result<()> {
    behave::logging::init("VideoEnc");
    info!("starting (H.264 encoder, {}x{})", WIDTH, HEIGHT);

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;

    let raw_sub = camera_raw_frame::subscribe(&node)?;
    info!("subscribed to {}", camera_raw_frame::NAME);

    let enc_pub = camera_encoded_frame::publish(&node)?;
    info!("publishing {}", camera_encoded_frame::NAME);

    let mut encoder = Encoder::new()?;
    info!("OpenH264 encoder initialized");

    let pixel_count = (WIDTH * HEIGHT) as usize;
    let mut rgb = vec![0u8; pixel_count * 3];
    let mut frame_count: u64 = 0;

    info!("ready");

    loop {
        // Use zero-copy receive to avoid stack-copying the 1.2MB frame
        match raw_sub.receive()? {
            Some(sample) => {
                let frame: &CameraRawFrame = &*sample;

                // BGRA -> RGB
                for i in 0..pixel_count {
                    let src = i * 4;
                    let dst = i * 3;
                    rgb[dst]     = frame.data[src + 2]; // R
                    rgb[dst + 1] = frame.data[src + 1]; // G
                    rgb[dst + 2] = frame.data[src];     // B
                }

                let rgb_source = RgbSliceU8::new(&rgb, (frame.width as usize, frame.height as usize));
                let yuv = YUVBuffer::from_rgb_source(rgb_source);
                let bitstream = encoder.encode(&yuv)?;

                let nal_data = bitstream.to_vec();

                if !nal_data.is_empty() {
                    topics::publish_bytes::<{ camera_encoded_frame::BUF }>(&enc_pub, &nal_data)?;
                    frame_count += 1;
                    if frame_count % 30 == 0 {
                        info!("encoded {frame_count} frames (last: {} bytes)", nal_data.len());
                    }
                }
            }
            None => {
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    }
}
