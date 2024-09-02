use std::fs::OpenOptions;
use std::io::Write;
mod v4l2;

const DEVICE_NAME: &str = "/dev/video0";

fn main() {
    let device = v4l2::V4L2VideoDevice::new(&DEVICE_NAME);

    let frame = device.get_frame();

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open("test.yuv")
        .unwrap();
    output.write_all(frame.data()).unwrap();
}
