use nokhwa::{
    Camera, NokhwaError,
    pixel_format::RgbFormat,
    utils::{CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType},
};
use std::sync::{Arc, Mutex, atomic::AtomicBool};

// Main camera data. Dervied fields come from camera.format
pub struct CameraData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub frame_format: FrameFormat,
}

// Shared state between CameraData recieved and egui
pub type SharedData = Arc<Mutex<Option<CameraData>>>;

fn set_up_camera() -> Result<Camera, NokhwaError> {
    println!("Starting camera...");

    let index = CameraIndex::Index(0);
    let request_format_type =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    println!("Creating camera object...");
    let mut camera = match Camera::new(index, request_format_type) {
        Ok(cam) => cam,
        Err(e) => {
            eprintln!("Error creating camera: {}", e);
            return Err(e);
        }
    };

    match camera.open_stream() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error opening stream: {}", e);
            return Err(e);
        }
    }
    Ok(camera)
}

pub fn capture_camera_data(
    camera_data: Arc<Mutex<Option<CameraData>>>,
    running: Arc<AtomicBool>,
) -> Result<(), NokhwaError> {
    let mut camera = match set_up_camera() {
        Ok(cam) => cam,
        Err(e) => {
            eprintln!("Error setting up camera: {e}");
            return Err(e);
        }
    };

    while running.load(std::sync::atomic::Ordering::Relaxed) {
        match camera.frame() {
            Ok(buffer) => {
                let frame_data = CameraData {
                    width: buffer.resolution().width(),
                    height: buffer.resolution().height(),
                    data: buffer.decode_image::<RgbFormat>().unwrap().into_raw(),
                    frame_format: buffer.source_frame_format(),
                };

                let mut lock = camera_data.lock().unwrap();
                *lock = Some(frame_data);
            }
            Err(e) => {
                eprintln!("Error capturing frame: {e}");
                break;
            }
        }
    }

    match camera.stop_stream() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error closing the camera stream: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
