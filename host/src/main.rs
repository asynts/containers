// Launch system in isolated environment.

// FIXME: Maybe rewrite this in C?

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

    // This will be used by 'pivot_root'.
    std::fs::create_dir(std::path::Path::new(CONTAINER_DIRECTORY).join("root")).unwrap();
}

extern "C"
fn child_main(_: *mut libc::c_void) -> libc::c_int {
    // Set propapation type of all mounts.  Since we ran 'clone' with 'CLONE_NEWNS', this will
    // not affect anyone else.
    unsafe {
        let root_cstring = std::ffi::CString::new("/").unwrap();
        let retval = libc::mount(std::ptr::null(), root_cstring.as_ptr(), std::ptr::null(), libc::MS_REC | libc::MS_PRIVATE, std::ptr::null());
        assert_eq!(retval, 0);
    }

    // To be able to utilize 'pivot_root', the target directory needs to be a mount point.
    unsafe {
        let source_cstring = std::ffi::CString::new(CONTAINER_DIRECTORY).unwrap();
        let retval = libc::mount(source_cstring.as_ptr(), source_cstring.as_ptr(), std::ptr::null(), libc::MS_BIND, std::ptr::null());
        assert_eq!(retval, 0);
    }

    // Now we swap the mount points, the old root becomes '/tmp/asynts-containers/root' the new
    // root becomes '/tmp/asynts-containers'.
    unsafe {
        let newroot_cstring = std::ffi::CString::new(CONTAINER_DIRECTORY).unwrap();
        let oldroot_cstring = std::ffi::CString::new("/tmp/asynts-containers/root").unwrap();
        let retval = libc::syscall(libc::SYS_pivot_root, newroot_cstring.as_ptr(), oldroot_cstring.as_ptr());
        assert_eq!(retval, 0);
    }

    // FIXME: I think this is implied by 'pivot_root'?
    std::env::set_current_dir("/").unwrap();

    // Now unmount the old root to make it disappear.
    unsafe {
        let target_cstring = std::ffi::CString::new("root").unwrap();
        let retval = libc::umount2(target_cstring.as_ptr(), libc::MNT_DETACH);
        assert_eq!(retval, 0);
    }

    std::fs::remove_dir("root").unwrap();

    println!("Prepared everything, handing over to application!");

    // We are now completely isolated from everything else.
    std::process::Command::new("./app")
        .env_clear()
        .exec();
    
    unreachable!();
}
fn create_container_process() -> i32 {
    let stack_size = 0x1000;
    let mut stack: Vec<u8> = vec![0; stack_size];

    unsafe {
        // FIXME: Do all the flags make sense?
        libc::clone(
            child_main,
            stack.as_mut_ptr().offset(stack_size.try_into().unwrap()) as *mut libc::c_void,
            libc::CLONE_NEWCGROUP | libc::CLONE_NEWIPC | libc::CLONE_NEWNET | libc::CLONE_NEWNS | libc::CLONE_NEWPID | libc::CLONE_NEWUSER | libc::CLONE_NEWUTS,
            std::ptr::null_mut()
        )
    }
}

fn main() {
    verify_has_root_privileges();

    prepare_container_directory();

    let container_process_id = create_container_process();

    // Wait for process to finish
    unsafe {
        while libc::waitpid(container_process_id, std::ptr::null_mut(), 0) != container_process_id {
        }
    }
}
