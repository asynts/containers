How does docker work?  That's what I am trying to figure out, this is a barebone
implementation of a container system.

The goal is to run an appication in an environment where it can not interact with
other processes or data.  At least provided that there aren't any issues in the
operating system.

There should really only be a single standalone executable that is able to access
the stdin and stdout of the parent jail process.

### Development Environment

-   The system itself needs to be statically linked, since no dynamic linker will be
    avaliable.  A suitable toolchain needs to be installed:

    ```none
    rustup target add x86_64-unknown-linux-musl
    ```

### Build Instructions

```none
./bootstrap.sh
```

### TODO

-   Redefine the scope:

    One simple `jail` executable that runs an application isolated from everything
    else.  The process that calls `jail` should not be privileged.

    My understanding is the following:

     1. First we use `unshare` to enter a new user namespace.

     2. We setup `/proc/self/uid_map` and `/proc/self/gid_map` to appear as
        root.

     3. Now, we should be able to use `clone` to enter new namespaces.

     4. We utilize `pivot_root` to hide the filesystem.

     5. We execute `execve` to run the application.

    Finally, I should verify that it is not possible to escape the jail.

-   Rust is really hindering me here, this should be a simple C++ application.

-   The following exploits work:

    ```none
    /bin/chw00t -0 --dir foo
    /bin/chw00t -1 --dir foo
    /bin/chw00t -2 --dir foo
    /bin/chw00t -5 --dir foo --tempdir bar --nestdir baz
    /bin/chw00t -9 --dir foo
    ```

    https://github.com/earthquake/chw00t

-   The following exploits may work:

    ```none
    /bin/chw00t -3 --dir foo
    /bin/chw00t -4 --dir foo --tempdir bar
    ```

    https://github.com/earthquake/chw00t
