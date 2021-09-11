### Development Environment

-   The system itself needs to be statically linked, since no dynamic linker will be
    avaliable.  Currently, this only seems to be possible with musl libc.  Let's install the
    toolchain:

    ~~~none
    rustup target add x86_64-unknown-linux-musl
    ~~~

### Build Instructions

-   First we need to build the system that will be run in an isolated environment:

    ~~~none
    cargo build --package asynts-containers-system --target x86_64-unknown-linux-musl
    ~~~

-   Now we can build and run the jail executable on the host system:

    ~~~none
    cargo run --package asynts-containers-host
    ~~~
