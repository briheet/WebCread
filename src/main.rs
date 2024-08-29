use std::fmt::Result;
use std::fs::OpenOptions;
use std::mem::MaybeUninit;
use std::os::fd::AsRawFd;

use v4l2::{v4l2_area, v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE, v4l2_memory_V4L2_MEMORY_USERPTR};

mod v4l2;

const DEVICE_NAME: &str = "/dev/video0";
const VIDIOC_QUERYCAP: u64 = 2154321408;
const VIDIOC_G_FMT: u64 = 3234878980;
const V4L2_PIX_FMT_MJPEG: u32 = 1196444237;
const VIDIOC_REQBUFS: u64 = 3222558216;

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
    assert!((capabilities.capabilities & v4l2::V4L2_CAP_STREAMING) != 0);

    let format = unsafe {
        let mut format: v4l2::v4l2_format = std::mem::zeroed();
        format.type_ = v4l2::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
        ioctl!(fd, VIDIOC_G_FMT, &mut format).unwrap();
        format
    };

    println!("{:?}", capabilities);
    unsafe {
        println!("image size: {:?}", format.fmt.pix.sizeimage);
        println!(
            "image width: {}, image height: {}, image pixel_format: {}",
            format.fmt.pix.width, format.fmt.pix.height, format.fmt.pix.pixelformat
        );
    }

    unsafe {
        assert!((format.fmt.pix.pixelformat & V4L2_PIX_FMT_MJPEG) != 0);
    }

    let image_size = unsafe { format.fmt.pix.sizeimage };

    const NUM_BUFFERS: u32 = 4;
    let mut bufs = Vec::new();

    for i in 0..NUM_BUFFERS {
        bufs.push(vec![0u8; image_size.try_into().unwrap()]);
    }

    unsafe {
        let mut buf: v4l2::v4l2_requestbuffers = std::mem::zeroed();
        buf.count = NUM_BUFFERS;
        buf.type_ = v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
        buf.memory = v4l2_memory_V4L2_MEMORY_USERPTR;

        ioctl!(fd, VIDIOC_REQBUFS, &mut buf).unwrap();
    }
}
