// Start target in isolated environment.

extern crate tempfile;

use std::os::unix::process::CommandExt;

fn main() {
    // FIXME: setup namespace
    // FIXME: all the other stuff

    let working_directory = tempfile::tempdir().unwrap();

    std::fs::copy(
        "/home/me/dev/containers/target/x86_64-unknown-linux-musl/debug/asynts-containers-system",
        working_directory.path().join("app")
    ).unwrap();

    std::os::unix::fs::chroot(working_directory.path()).unwrap();
    std::env::set_current_dir("/").unwrap();

    std::process::Command::new("./app")
        .exec();
    
    unreachable!();
}
