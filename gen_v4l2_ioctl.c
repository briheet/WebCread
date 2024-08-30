#include <stdio.h>

#include <linux/videodev2.h>

#define PRINT_IOCTL_NUMBER(n) printf("const %s: u64 = %lu;\n", #n, n);
#define PRINT_IOCTL_NUMBER_US(n) printf("const %s: u64 = %u;\n", #n, n)
int main(void) {
  PRINT_IOCTL_NUMBER(VIDIOC_QUERYCAP);
  PRINT_IOCTL_NUMBER(VIDIOC_G_FMT);
  PRINT_IOCTL_NUMBER(V4L2_PIX_FMT_MJPEG);
  PRINT_IOCTL_NUMBER(VIDIOC_REQBUFS);
  PRINT_IOCTL_NUMBER(VIDIOC_QBUF);
  PRINT_IOCTL_NUMBER_US(V4L2_BUF_TYPE_VIDEO_CAPTURE);
  PRINT_IOCTL_NUMBER_US(V4L2_MEMORY_USERPTR);
  PRINT_IOCTL_NUMBER(VIDIOC_STREAMON);
  PRINT_IOCTL_NUMBER(VIDIOC_DQBUF);
}
