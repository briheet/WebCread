use std::fs::OpenOptions;
use std::mem::MaybeUninit;
use std::os::fd::AsRawFd;

use v4l2::v4l2_capability;
mod v4l2;

const VIDIOC_QUERYCAP: u64 = 2154321408;
const DEVICE_NAME: &str = "/dev/video0";
fn main() {
    let video_handle = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DEVICE_NAME)
        .unwrap();

    let fd = video_handle.as_raw_fd();

    let mut capabilities: MaybeUninit<v4l2_capability> = MaybeUninit::uninit();

    unsafe {
        v4l2::ioctl(fd, VIDIOC_QUERYCAP, capabilities.as_mut_ptr());
    }

    println!("Hello, world!");
}
