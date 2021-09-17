commitid 4b525cbac85155ce7316c5edba236dc6b1b5a052

The `clone()` call seems to succeed, but we immediatelly segfault in the child process.

### Notes

-   After removing the `println!` statement, everything seems to work.  It appears, that
    the rust runtime can not deal with `fork()` or `clone()` properly.

### Ideas

-   Try without the `println!` statement.

### Theories

-   There is a `println!` statement in the first line, maybe some state was destroyed
    by the `clone()`?

### Actions

-   Implement `child_main` in C instead of Rust.
