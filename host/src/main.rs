// Launch system in isolated environment.

extern crate tempfile;
extern crate libc;

use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;

fn verify_has_root_privileges() {
    unsafe {
        if libc::geteuid() != 0 {
            panic!();
        }
    }
}

// FIXME: Can I make this a std::path::Path object?
const CONTAINER_DIRECTORY: &str = "/tmp/asynts-containers";

fn prepare_container_directory() {
    if std::path::Path::new(CONTAINER_DIRECTORY).exists() {
        std::fs::remove_dir_all(CONTAINER_DIRECTORY).unwrap();
    }

    std::fs::create_dir(CONTAINER_DIRECTORY).unwrap();
    std::fs::set_permissions(CONTAINER_DIRECTORY, std::fs::Permissions::from_mode(0o00700)).unwrap();

    std::fs::copy(
        "/home/me/dev/containers/target/x86_64-unknown-linux-musl/debug/asynts-containers-system",
        std::path::Path::new(CONTAINER_DIRECTORY).join("app")
    ).unwrap();
}

extern "C"
fn child_main(_: *mut libc::c_void) -> libc::c_int {
    // FIXME: Set mount type MS_SLAVE

    // FIXME: Mount CONTAINER_DIRECTORY?

    // FIXME: Call pivot_root

    std::os::unix::fs::chroot(CONTAINER_DIRECTORY).unwrap();
    std::env::set_current_dir("/").unwrap();

    std::process::Command::new("./app")
        .env_clear()
        .exec();
    
    unreachable!();
}
fn create_container_process() {
    let stack_size = 0x1000;
    let mut stack: Vec<u8> = vec![0; stack_size];

    // FIXME: Save process id
    unsafe {
        // FIXME: Do all the flags make sense?
        libc::clone(
            child_main,
            stack.as_mut_ptr().offset(stack_size.try_into().unwrap()) as *mut libc::c_void,
            libc::CLONE_NEWCGROUP | libc::CLONE_NEWIPC | libc::CLONE_NEWNET | libc::CLONE_NEWNS | libc::CLONE_NEWPID | libc::CLONE_NEWUSER | libc::CLONE_NEWUTS,
            std::ptr::null_mut()
        );
    }
}

fn main() {
    verify_has_root_privileges();

    prepare_container_directory();

    let _container_process = create_container_process();

    // FIXME: Wait for process to finish.
}
