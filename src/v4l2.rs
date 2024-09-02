mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/v4l2-bindings.rs"));
}

use std::convert::AsRef;
use std::fs::File;
use std::fs::OpenOptions;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::os::fd::AsRawFd;
use std::path::Path;

// use v4l2::{v4l2_area, v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE, v4l2_memory_V4L2_MEMORY_USERPTR};

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
        let ret = unsafe { sys::ioctl($fd, $num, $arg) };
        if ret == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(ret)
        }
    }};
}

pub struct V4L2VideoDevice {
    handle: File,
    _bufs: Vec<Vec<u8>>,
}

pub struct V4L2Frame<'a> {
    fd: i32,
    buf: sys::v4l2_buffer,
    _phantom: PhantomData<&'a ()>,
}

impl V4L2Frame<'_> {
    pub fn width(&self) -> usize {
        // FIXME: Get the actual rather than hardcoing
        640
    }

    pub fn height(&self) -> usize {
        // FIXME: Get the actual rather than hardcoing
        480
    }
    pub fn data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.buf.m.userptr as *const u8, self.buf.length as usize)
        }
    }
}

impl Drop for V4L2Frame<'_> {
    fn drop(&mut self) {
        unsafe {
            ioctl!(self.fd, VIDIOC_QBUF, &mut self.buf).unwrap();
        }
    }
}

impl V4L2VideoDevice {
    pub fn new<P: AsRef<Path>>(device_path: &P) -> V4L2VideoDevice {
        let video_handle = OpenOptions::new()
            .read(true)
            .write(true)
            .open(device_path)
            .unwrap();

        let fd = video_handle.as_raw_fd();

        let capabilities = unsafe {
            let mut capabilities: MaybeUninit<sys::v4l2_capability> = MaybeUninit::uninit();
            ioctl!(fd, VIDIOC_QUERYCAP, capabilities.as_mut_ptr()).unwrap();
            capabilities.assume_init()
        };

        assert!((capabilities.capabilities & sys::V4L2_CAP_VIDEO_CAPTURE) != 0);
        // it does not have the read wite -> ./test -r
        assert!((capabilities.capabilities & sys::V4L2_CAP_STREAMING) != 0);

        let format = unsafe {
            let mut format: sys::v4l2_format = std::mem::zeroed();
            format.type_ = sys::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
            ioctl!(fd, VIDIOC_G_FMT, &mut format).unwrap();
            format
        };

        // println!("{:?}", capabilities);
        unsafe {
            println!("image size: {:?}", format.fmt.pix.sizeimage);
            println!(
                "image width: {}, image height: {}, image pixel_format: {}, image sizeimage: {}",
                format.fmt.pix.width,
                format.fmt.pix.height,
                format.fmt.pix.pixelformat,
                format.fmt.pix.sizeimage
            );
        }

        unsafe {
            assert!((format.fmt.pix.pixelformat & V4L2_PIX_FMT_YUYV) != 0);
        }

        let image_size = unsafe { format.fmt.pix.sizeimage };

        const NUM_BUFFERS: u32 = 4;
        // FIXME: unsafe cell around each bufs
        let mut bufs = Vec::new();

        for _i in 0..NUM_BUFFERS {
            bufs.push(vec![0u8; image_size.try_into().unwrap()]);
        }

        unsafe {
            let mut buf: sys::v4l2_requestbuffers = std::mem::zeroed();
            buf.count = NUM_BUFFERS;
            buf.type_ = sys::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
            buf.memory = sys::v4l2_memory_V4L2_MEMORY_USERPTR;

            ioctl!(fd, VIDIOC_REQBUFS, &mut buf).unwrap();
        }

        for i in 0..NUM_BUFFERS {
            unsafe {
                let mut v4l2_buf: sys::v4l2_buffer = std::mem::zeroed();

                // let buf = &bufs[i as usize];
                v4l2_buf.type_ = V4L2_BUF_TYPE_VIDEO_CAPTURE;
                v4l2_buf.index = i;
                v4l2_buf.memory = V4L2_MEMORY_USERPTR;
                v4l2_buf.m.userptr = bufs[i as usize].as_ptr() as u64;
                v4l2_buf.length = bufs[i as usize].len() as u32;
                ioctl!(fd, VIDIOC_QBUF, &v4l2_buf).unwrap();
            }
        }

        let video_cap_buf_type: sys::v4l2_buf_type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
        unsafe {
            ioctl!(fd, VIDIOC_STREAMON, &video_cap_buf_type).unwrap();
        }

        V4L2VideoDevice {
            handle: video_handle,
            _bufs: bufs,
        }
    }

    pub fn get_frame(&self) -> V4L2Frame<'_> {
        // automatic reference to function sig

        let fd = self.handle.as_raw_fd();
        let mut poll_fd: [sys::pollfd; 1] = [sys::pollfd {
            fd,
            events: sys::POLLIN as i16,
            revents: 0,
        }];

        unsafe {
            sys::poll(poll_fd.as_mut_ptr(), poll_fd.len() as u64, -1);

            let mut v4l2_buf: sys::v4l2_buffer = std::mem::zeroed();

            v4l2_buf.type_ = V4L2_BUF_TYPE_VIDEO_CAPTURE;
            v4l2_buf.memory = V4L2_MEMORY_USERPTR;

            ioctl!(fd, VIDIOC_DQBUF, &mut v4l2_buf).unwrap();

            V4L2Frame {
                fd,
                buf: v4l2_buf,
                _phantom: PhantomData,
            }
        }
    }
}
