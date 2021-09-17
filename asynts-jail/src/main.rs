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
    stack: [u8; 0x1000],
    child_pid: Option<nix::unistd::Pid>,
    child_arguments: Option<asynts_jail_sys::ChildArguments>,
}
impl Service {
    fn new() -> Service {
        Service{
            directory: None,
            stack: [0; 0x1000],
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
        assert!(matches!(status, nix::sys::wait::WaitStatus::Exited(_, _)));
    }

    fn _directory(&self) -> &std::path::Path {
        self.directory.as_ref().unwrap().path()
    }

    fn _prepare_directory(&mut self) {
        assert!(self.directory.is_none());
        self.directory = Some(
            tempfile::Builder::new()
                .prefix("jail.")
                .tempdir()
                .unwrap()
        );

        std::fs::create_dir(self._directory().join("bin")).unwrap();

        std::fs::copy(
            std::env::current_exe().unwrap().parent().unwrap().join("../x86_64-unknown-linux-musl/debug/asynts-jail-example"),
            self._directory().join("bin/init")
        ).unwrap();
    }

    fn _spawn_application_process(&mut self) {
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
                    &mut self.stack,
                    flags,
                    Some(libc::SIGCHLD)
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
