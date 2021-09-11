// Runs in jail; should not have access to anything.

#include <stdio.h>
#include <ftw.h>
#include <assert.h>

static int callback_nftw(
    const char *filepath,
    const struct stat *stat,
    int flags,
    struct FTW *ftw)
{
    printf("%s\n", filepath);

    return 0;
}

int main() {
    int retval = nftw("/", callback_nftw, 0);
    assert(retval == 0);
}
