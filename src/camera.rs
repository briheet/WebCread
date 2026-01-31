use nokhwa::{
    Camera,
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
};

fn get_camera_data() {
    println!("Starting camera...");

    let index = CameraIndex::Index(0);
    let request_format_type =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);

    println!("Creating camera object...");
    let mut camera = match Camera::new(index, request_format_type) {
        Ok(cam) => cam,
        Err(e) => {
            eprintln!("Error creating camera: {}", e);
            return;
        }
    };

    camera.open_stream().unwrap();

    println!("Capturing frame...");
    let frame = match camera.frame() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error capturing frame: {}", e);
            return;
        }
    };

    println!("Captured frame: {}", frame.resolution());

    camera.stop_stream().unwrap();
}
