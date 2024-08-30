use std::fs::OpenOptions;
use std::io::Write;
use std::mem::MaybeUninit;
use std::os::fd::AsRawFd;
use std::{fmt::Result, thread};

use libc::ioctl;
use v4l2::{v4l2_area, v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE, v4l2_memory_V4L2_MEMORY_USERPTR};

mod v4l2;

const DEVICE_NAME: &str = "/dev/video0";
const VIDIOC_QUERYCAP: u64 = 2154321408;
const VIDIOC_G_FMT: u64 = 3234878980;
const V4L2_PIX_FMT_MJPEG: u32 = 1196444237;
const VIDIOC_REQBUFS: u64 = 3222558216;
const VIDIOC_QBUF: u64 = 3227014671;
const V4L2_BUF_TYPE_VIDEO_CAPTURE: u32 = 1;
const V4L2_MEMORY_USERPTR: u32 = 2;
const VIDIOC_STREAMON: u64 = 1074026002;
const VIDIOC_DQBUF: u64 = 3227014673;
const V4L2_PIX_FMT_YUYV: u32 = 1448695129;

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
        assert!((format.fmt.pix.pixelformat & V4L2_PIX_FMT_YUYV) != 0);
    }

    let image_size = unsafe { format.fmt.pix.sizeimage };

    const NUM_BUFFERS: u32 = 4;
    // FIXME: unsafe cell around each bufs
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

    for i in 0..NUM_BUFFERS {
        unsafe {
            let mut v4l2_buf: v4l2::v4l2_buffer = std::mem::zeroed();

            let buf = &bufs[i as usize];
            v4l2_buf.type_ = V4L2_BUF_TYPE_VIDEO_CAPTURE;
            v4l2_buf.index = i;
            v4l2_buf.memory = V4L2_MEMORY_USERPTR;
            v4l2_buf.m.userptr = bufs[i as usize].as_ptr() as u64;
            v4l2_buf.length = bufs[i as usize].len() as u32;
            ioctl!(fd, VIDIOC_QBUF, &v4l2_buf).unwrap();
        }
    }

    // drop(bufs);

    let video_cap_buf_type: v4l2::v4l2_buf_type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
    unsafe {
        ioctl!(fd, VIDIOC_STREAMON, &video_cap_buf_type).unwrap();
    }

    loop {
        unsafe {
            let mut poll_fd: [v4l2::pollfd; 1] = [v4l2::pollfd {
                fd,
                events: v4l2::POLLIN as i16,
                revents: 0,
            }];

            println!(
                "{}",
                v4l2::poll(poll_fd.as_mut_ptr(), poll_fd.len() as u64, -1)
            );

            let mut v4l2_buf: v4l2::v4l2_buffer = std::mem::zeroed();

            v4l2_buf.type_ = V4L2_BUF_TYPE_VIDEO_CAPTURE;
            v4l2_buf.memory = V4L2_MEMORY_USERPTR;

            ioctl!(fd, VIDIOC_DQBUF, &mut v4l2_buf).unwrap();
            let buf = std::slice::from_raw_parts(
                v4l2_buf.m.userptr as *const u8,
                v4l2_buf.length as usize,
            );
            let mut output = OpenOptions::new()
                .write(true)
                .create(true)
                .open("test.yuv")
                .unwrap();
            output.write_all(&buf).unwrap();

            ioctl!(fd, VIDIOC_QBUF, &mut v4l2_buf).unwrap();
            return;
            // println!("{}", rgb.len());
            // println!("{:?}", decoder.info().unwrap());
        }
    }
}
