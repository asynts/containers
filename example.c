#include <stdio.h>
#include <unistd.h>
#include <sys/stat.h>
#include <stdlib.h>
#include <assert.h>

void check_if_escaped() {
    // First we try to navigate to the real root and chroot there.
    {
        for (int i = 0; i < 32; ++i)
            chdir("..");

        {
            int retval = chroot(".");
            assert(retval == 0);
        }
    }

    // Since there is no '/proc' in the jail, this is enough to tell if we
    // escaped or not.
    {
        struct stat statbuf;
        if (stat("/proc", &statbuf) == 0) {
            printf("escaped!\n");
        } else {
            printf("did not escape!\n");
        }
    }
}

void test_method_0() {
}

void test_method_1() {
    {
        int retval = mkdir("foo", 0777);
        assert(retval == 0);
    }

    {
        int retval = chroot("foo");
        assert(retval == 0);
    }
}

int main() {
    test_method_0();
    // test_method_1();
    check_if_escaped();
}
