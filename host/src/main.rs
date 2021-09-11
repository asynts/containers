// Start target in isolated environment.

extern crate tempfile;

use std::os::unix::process::CommandExt;

    // let working_directory = /* ... */;
    // chmod(/* ... */);

    // unshare(CLONE_NEWCGROUP | CLONE_NEWIPC | CLONE_NEWNET | CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWTIME | CLONE_NEWUSER | CLONE_NEWUTS)
    // // Not all 'unshare' operations can be applied directly, we need to create a new process
    // // that does the required operations.
    // fork()

    // // If we don't do this, we could modify the mounts of the host.  This appears to be
    // // necessary for 'pivot_root'.
    // mount(NULL, working_directory, NULL, MS_SLAVE, NULL);

    // // This should prevent a super user from escaping the chroot.
    // chdir(working_directory)
    // pivot_root(.)
    // chroot(.)

    // execve(/* ... */)

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

    // FIXME: We are still root here!

    std::process::Command::new("./app")
        .exec();
    
    unreachable!();
}
