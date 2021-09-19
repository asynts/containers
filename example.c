// This application will be run in a jail where it can only interact with
// the remaining system via stdin and stdout.
//
// From the perspective of the application, it will be the only process
// runing on the system with PID=1, UID=0 and GID=0.
//
// If the user that runs 'jail' has supplementary groups, then they will
// appear as group 'nobody' with GID=65534.
//
// The application itself is located at '/sbin/init' and is the only file
// that is exposed.  Since there is no dynamic linker or any libraries
// avaliable, this executable needs to be statically linked and must not
// rely on filesystems such as '/tmp' or '/proc' to be present.

#include <stdio.h>

int main() {
    printf("Hello, world!\n");
}
