#include <stdio.h>

#include <linux/videodev2.h>

#define PRINT_IOCTL_NUMBER(n) printf("const %s: u64 = %lu;\n", #n, n);
int main(void) { PRINT_IOCTL_NUMBER(VIDIOC_QUERYCAP); }
