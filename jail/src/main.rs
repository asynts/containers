extern crate libc;
extern crate tempfile;

use std::io::prelude::*;

mod util {
    pub fn is_root_user() -> bool {
        unsafe { libc::geteuid() == 0 }
    }
}

struct Stack {
    _buffer: [u8; 0x1000]
}
impl Stack {
    fn new() -> Stack {
        Stack{
            _buffer: [0; 0x1000]
        }
    }

    fn top(&mut self) -> *mut u8 {
        unsafe { self._buffer.as_mut_ptr().offset(0x1000) }
    }
}

struct Service {
    directory: Option<tempfile::TempDir>,
    stack: Option<Stack>,
    child_pid: Option<i32>,
}
impl Service {
    fn new() -> Service {
        Service{
            directory: None,
            stack: None,
            child_pid: None
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
        // FIXME: Currently, the executable is not statically linked
        std::fs::copy(
            "/home/me/dev/jail/target/x86_64-unknown-linux-musl/debug/asynts-example",
            self.directory.as_ref().unwrap().path().join("application")
        ).unwrap();
    }

    fn _spawn_application_process(&mut self) {
        assert!(self.stack.is_none());
        self.stack = Some(Stack::new());

        extern "C" fn child_main(_: *mut libc::c_void) -> libc::c_int {
            // FIXME: Somehow, we SIGSEGV here?
            //        https://github.com/rust-lang/rust/blob/d1d8145dffde1092135b571d1d19205fe2a8fc44/library/std/src/io/stdio.rs#L1205
            println!("Executing in child process!");
            0
        }

        unsafe {
            // NOTE: We do not create a new mount or network namespace.  This is, because
            // we want to be able to share these between services.
            let retval = libc::clone(
                child_main,
                self.stack.as_mut().unwrap().top() as *mut libc::c_void,
                libc::CLONE_NEWCGROUP | libc::CLONE_NEWIPC | libc::CLONE_NEWPID | libc::CLONE_NEWUSER | libc::CLONE_NEWUTS,
                std::ptr::null_mut()
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
