extern crate libc;
extern crate tempfile;
extern crate nix;
extern crate asynts_jail_sys;

use std::io::prelude::*;

mod util {
    pub fn is_root_user() -> bool {
        nix::unistd::geteuid().is_root()
    }
}

// Services are run in a jail, where they can not interact with other parts of the
// system.  This isolation does not utilize a proper virtual machine, but relies on
// functionality in the Linux kernel instead.
struct Service {
    directory: Option<tempfile::TempDir>,
    stack: Option<asynts_jail_sys::ChildStack>,
    child_pid: Option<nix::unistd::Pid>,
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

    fn wait(&self) {
        let status = nix::sys::wait::waitpid(self.child_pid, None).unwrap();

        // FIXME: This fails, because the process is already dead before we are
        //        here.  Not sure why this would be an issue, but I suspect, that
        //        the Rust runtime already ate that event.
        assert!(matches!(status, nix::sys::wait::WaitStatus::Exited(_, _)));
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
            "/home/me/dev/jail/target/x86_64-unknown-linux-musl/debug/asynts-jail-example",
            self.directory.as_ref().unwrap().path().join("application")
        ).unwrap();
    }

    fn _spawn_application_process(&mut self) {
        // FIXME: Get rid of that 'ChildStack' class, just use a raw buffer.
        assert!(self.stack.is_none());
        self.stack = Some(asynts_jail_sys::ChildStack::new());

        unsafe {
            self.child_arguments = Some(asynts_jail_sys::ChildArguments::new(self.directory.as_ref().unwrap().path().to_str().unwrap()));

            let flags = nix::sched::CloneFlags::CLONE_NEWCGROUP
                      | nix::sched::CloneFlags::CLONE_NEWIPC
                      | nix::sched::CloneFlags::CLONE_NEWPID
                      | nix::sched::CloneFlags::CLONE_NEWUSER
                      | nix::sched::CloneFlags::CLONE_NEWUTS
                      | nix::sched::CloneFlags::CLONE_NEWNET
                      | nix::sched::CloneFlags::CLONE_NEWNS;

            let callback = Box::new(|| asynts_jail_sys::child_main(self.child_arguments.as_mut().unwrap().ffi()));

            self.child_pid = Some(
                nix::sched::clone(
                    callback,
                    &mut self.stack.as_mut().unwrap().buffer,
                    flags,
                    None
                ).unwrap()
            );
        }
    }
}

fn main() {
    // Many operations require the 'CAP_SYS_ADMIN' capability, this check is
    // not too far off.
    assert!(util::is_root_user());

    let mut service = Service::new();
    service.launch();

    std::io::stdout().flush().unwrap();
    service.wait();
}
