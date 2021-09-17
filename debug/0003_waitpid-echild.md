commitid a10e374022609272bf146183a101d5e59ee69c30

The `waitpid()` call returns an `ECHILD` error.

### Notes

-   It appears, the `waitpid` fails before `child_main` runs, or very close to it.

-   If we add a delay before the `waitpid` call, everything works.

-   The documentation of `clone()` states, that it returns the TID not the
    PID.  Not sure if this is correct though.

-   In a simple test case, it appeared to work with `SIGCHILD` in the flags.
    https://paste.ee/p/OhD8e

### Ideas

-   Make sure that all the `libc` stuff is gone.

-   Create a minimal example.

### Theories

-   I think it only worked by coincidence before, because both PID and TID were 1.

### Actions

-   Used `nix` crate instead of `libc`.

-   Pass `SIGCHLD` to `clone()`.
