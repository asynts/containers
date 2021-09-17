commitid a10e374022609272bf146183a101d5e59ee69c30

The `waitpid()` call returns an `ECHILD` error.

### Notes

### Ideas

-   Make sure that all the `libc` stuff is gone.

-   What does the `signal` argument to `clone()` mean, I've seen people pass
    `SIGCHLD` there?

### Theories

-   I suspect, that the Rust runtime is eating the event that `waitpid()`
    would pickup otherwise.

### Actions

-   Used `nix` crate instead of `libc`.
