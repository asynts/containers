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
    lock_fd: Option<std::os::unix::io::RawFd>,
}
impl Service {
    fn new() -> Service {
        Service{
            directory: None,
            stack: [0; 0x1000],
            child_pid: None,
            child_arguments: None,
            lock_fd: None,
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

        let lock_path = self._directory().join("lock");

        self.lock_fd = Some(
            nix::fcntl::open(
                lock_path.as_path(),
                nix::fcntl::OFlag::O_CREAT | nix::fcntl::OFlag::O_WRONLY | nix::fcntl::OFlag::O_CLOEXEC,
                nix::sys::stat::Mode::S_IRUSR | nix::sys::stat::Mode::S_IWUSR
            ).unwrap()
        );

        // The child will wait for this lock until we give the go-ahead.
        nix::fcntl::flock(self.lock_fd.unwrap(), nix::fcntl::FlockArg::LockExclusive).unwrap();

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

            self._configure_uid_mappings();
            self._configure_gid_mappings();

            // We have everything prepared, the child process can startup.
            nix::fcntl::flock(self.lock_fd.unwrap(), nix::fcntl::FlockArg::Unlock).unwrap();

            nix::unistd::close(self.lock_fd.unwrap()).unwrap();
            self.lock_fd = None;
        }
    }

    fn _configure_uid_mappings(&mut self) {
        let uidmap_path = format!("/proc/{}/uid_map", self.child_pid.unwrap().as_raw());

        let uidmap_fd = nix::fcntl::open(
            uidmap_path.as_str(),
            nix::fcntl::OFlag::O_WRONLY,
            nix::sys::stat::Mode::empty()
        ).unwrap();

        // FIXME: This may be problematic, because the new process will have UID=0 in the
        //        parent namespace.
        let uidmap_contents = format!("0 {} 1", nix::unistd::geteuid().as_raw());

        nix::unistd::write(uidmap_fd, uidmap_contents.as_bytes()).unwrap();

        nix::unistd::close(uidmap_fd).unwrap();
    }

    fn _configure_gid_mappings(&mut self) {
        let gidmap_path = format!("/proc/{}/gid_map", self.child_pid.unwrap().as_raw());

        let gidmap_fd = nix::fcntl::open(
            gidmap_path.as_str(),
            nix::fcntl::OFlag::O_WRONLY,
            nix::sys::stat::Mode::empty()
        ).unwrap();

        // FIXME: This may be problematic, because the new process will have GID=0 in the
        //        parent namespace.
        let gidmap_contents = format!("0 {} 1", nix::unistd::getegid().as_raw());

        nix::unistd::write(gidmap_fd, gidmap_contents.as_bytes()).unwrap();

        nix::unistd::close(gidmap_fd).unwrap();
    }
}

fn main() {
    // Many operations require the 'CAP_SYS_ADMIN' capability, this check is
    // not too far off.
    assert!(util::is_root_user());

    println!("jail: Executing as UID {}", nix::unistd::geteuid().as_raw());

    let mut service = Service::new();
    service.launch();

    std::io::stdout().flush().unwrap();
    service.wait();
}
