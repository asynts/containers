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

-   Now we can build jail executable that prepares the enviroment and executes the system:

    ~~~none
    cargo build --package asynts-containers-host
    ~~~

-   Now, we can run the system:

    ~~~none
    sudo ./target/debug/asynts-containers-host
    ~~~
