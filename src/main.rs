use std::fs::OpenOptions;
mod v4l2;

const DEVICE_NAME: &str = "/dev/video0";
fn main() {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DEVICE_NAME)
        .unwrap();

    println!("Hello, world!");
}
