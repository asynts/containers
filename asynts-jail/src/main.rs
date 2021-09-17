extern crate libc;
extern crate tempfile;
extern crate asynts_jail_sys;

use std::io::prelude::*;

mod util {
    pub fn is_root_user() -> bool {
        unsafe { libc::geteuid() == 0 }
    }
}

// Services are run in a jail, where they can not interact with other parts of the
// system.  This isolation does not utilize a proper virtual machine, but relies on
// functionality in the Linux kernel instead.
struct Service {
    directory: Option<tempfile::TempDir>,
    stack: Option<asynts_jail_sys::ChildStack>,
    child_pid: Option<i32>,
    child_arguments: Option<asynts_jail_sys::ChildArguments>,
}
impl Service {
    fn new() -> Service {
        Service{
            directory: None,
            stack: None,
            child_pid: None,
            child_arguments: None,
        }
    }

    fn launch(&mut self) {
        self._prepare_directory();
        self._spawn_application_process();
    }

    fn _prepare_directory(&mut self) {
        assert!(self.directory.is_none());
        self.directory = Some(
            tempfile::Builder::new()
                .prefix("jail.")
                .tempdir()
                .unwrap()
        );

        // FIXME: Don't hardcode path to executable
        std::fs::copy(
            "/home/me/dev/jail/target/x86_64-unknown-linux-musl/debug/asynts-example",
            self.directory.as_ref().unwrap().path().join("application")
        ).unwrap();
    }

    fn _spawn_application_process(&mut self) {
        assert!(self.stack.is_none());
        self.stack = Some(asynts_jail_sys::ChildStack::new());

        unsafe {
            self.child_arguments = Some(asynts_jail_sys::ChildArguments::new(self.directory.as_ref().unwrap().path().to_str().unwrap()));

            // FIXME: We might want to share the mount and network namespaces.
            let retval = libc::clone(
                asynts_jail_sys::child_main,
                self.stack.as_mut().unwrap().top() as *mut libc::c_void,
                libc::CLONE_NEWCGROUP | libc::CLONE_NEWIPC | libc::CLONE_NEWPID | libc::CLONE_NEWUSER | libc::CLONE_NEWUTS | libc::CLONE_NEWNET | libc::CLONE_NEWNS,
                self.child_arguments.as_mut().unwrap().ffi()
            );

            assert!(retval >= 0);
            self.child_pid = Some(retval);
        }
    }
}

fn main() {
    // Many operations require the 'CAP_SYS_ADMIN' capability, this check is
    // not too far off.
    assert!(util::is_root_user());

    let mut service = Service::new();
    service.launch();

    println!("directory: {:?}", service.directory.as_ref().unwrap().path());
    println!("child_pid: {}", service.child_pid.unwrap());

    print!("Press ENTER to continue...");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
