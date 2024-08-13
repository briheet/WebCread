#include <stdio.h>

#include <linux/videodev2.h>

#define PRINT_IOCTL_NUMBER(n) printf("const %s: u64 = %lu;\n", #n, n);
int main(void) {
  PRINT_IOCTL_NUMBER(VIDIOC_QUERYCAP);
  PRINT_IOCTL_NUMBER(VIDIOC_G_FMT);
  PRINT_IOCTL_NUMBER(V4L2_PIX_FMT_MJPEG);
}
