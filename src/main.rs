use std::fmt::Result;
use std::fs::OpenOptions;
use std::mem::MaybeUninit;
use std::os::fd::AsRawFd;

mod v4l2;

const VIDIOC_QUERYCAP: u64 = 2154321408;
const DEVICE_NAME: &str = "/dev/video0";

macro_rules! ioctl {
    ($fd:expr, $num:expr, $arg:expr) => {{
        let ret = unsafe { v4l2::ioctl($fd, $num, $arg) };
        if ret == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(ret)
        }
    }};
}
fn main() {
    let video_handle = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DEVICE_NAME)
        .unwrap();

    let fd = video_handle.as_raw_fd();

    let capabilities = unsafe {
        let mut capabilities: MaybeUninit<v4l2::v4l2_capability> = MaybeUninit::uninit();
        ioctl!(fd, VIDIOC_QUERYCAP, capabilities.as_mut_ptr()).unwrap();
        capabilities.assume_init()
    };

    assert!((capabilities.capabilities & v4l2::V4L2_CAP_VIDEO_CAPTURE) != 0);
    // it does not have the read wite -> ./test -r
    // assert!((capabilities.capabilities & v4ls::V4L2_CAP_READWRITE));

    println!("{:?}", capabilities);
}
