commitid a10e374022609272bf146183a101d5e59ee69c30

The `waitpid()` call returns an `ECHILD` error.

### Notes

-   It appears, the `waitpid` fails before `child_main` runs, or very close to it.

-   If we add a delay before the `waitpid` call, everything works.

### Ideas

-   Make sure that all the `libc` stuff is gone.

-   Create a minimal example.

### Theories

### Actions

-   Used `nix` crate instead of `libc`.
