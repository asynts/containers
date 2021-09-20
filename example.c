#define _GNU_SOURCE

#include <stdio.h>
#include <ftw.h>
#include <assert.h>
#include <unistd.h>

static int filepath_callback(
    const char *fpath,
    const struct stat *sb,
    int typeflag,
    struct FTW *ftwbuf)
{
    printf("  %s\n", fpath);
    return 0;
}

int main() {
    // We appear to be the root user.
    {
        printf("UID=%i (eUID=%i)\n", getuid(), geteuid());
        printf("GID=%i (eGID=%i)\n", getgid(), getegid());
    }

    // We appear to be the init process.
    {
        printf("PID=%i PPID=%i\n", getpid(), getppid());
    }

    // We can only see our own executable at '/sbin/init'.
    {
        printf("Walking directory '/':\n");
        int retval = nftw("/", filepath_callback, 0, 0);
        assert(retval == 0);
    }
}
