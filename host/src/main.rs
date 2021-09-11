// Start target in isolated environment.

use std::os::unix::process::CommandExt;

fn main() {
    // FIXME: setup namespace
    // FIXME: all the other stuff

    std::os::unix::fs::chroot("/home/me/dev/containers/target/x86_64-unknown-linux-musl/debug/").unwrap();
    std::env::set_current_dir("/").unwrap();

    std::process::Command::new("./asynts-containers-system")
        .exec();
    
    unreachable!();
}
