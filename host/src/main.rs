// Launch system in isolated environment.

extern crate tempfile;
extern crate libc;

use std::os::unix::fs::PermissionsExt;
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

fn create_container_directory() -> tempfile::TempDir {
    // We put the executable into a directory that is only accessible by us.
    let working_directory = tempfile::Builder::new()
        .prefix("container.")
        .rand_bytes(8)
        .tempdir()
        .unwrap();
    std::fs::set_permissions(working_directory.path(), std::fs::Permissions::from_mode(0o00700)).unwrap();

    working_directory
}

extern "C"
fn child_main(_: *mut libc::c_void) -> libc::c_int {
    std::process::Command::new("./app")
        .env_clear()
        .exec();
    
    unreachable!();
}
fn create_container_process(_container_directory: &tempfile::TempDir) {
    let stack_size = 0x1000;
    let mut stack: Vec<u8> = vec![0; stack_size];

    // FIXME: We don't use '_container_directory' here, but we have to.
    //        Maybe we could fork() + unshare() + fork()
    //
    //        Alternatively, we could destroy the parent process.

    unsafe {
        libc::clone(
            child_main,
            stack.as_mut_ptr().offset(stack_size.try_into().unwrap()) as *mut libc::c_void,
            libc::CLONE_NEWCGROUP | libc::CLONE_NEWIPC | libc::CLONE_NEWNET | libc::CLONE_NEWNS | libc::CLONE_NEWPID | libc::CLONE_NEWUSER | libc::CLONE_NEWUTS,
            std::ptr::null_mut()
        );
    }
}

fn main() {
    // Many of the operations here require root pivileges.
    unsafe {
        if libc::geteuid() != 0 {
            panic!();
        }
    }

    let container_directory = create_container_directory();
    let _container_process = create_container_process(&container_directory);

    // FIXME: Can we communicate with the process?
}
