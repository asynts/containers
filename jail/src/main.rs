extern crate libc;
extern crate tempfile;

use std::io::prelude::*;

mod util {
    pub fn is_root_user() -> bool {
        unsafe { libc::geteuid() == 0 }
    }
}

struct Service {
    directory: Option<tempfile::TempDir>
}

impl Service {
    fn new() -> Service {
        Service{ directory: None }
    }

    fn launch(&mut self) {
        self._prepare_directory();
    }

    fn _prepare_directory(&mut self) {
        self.directory = Some(
            tempfile::Builder::new()
                .prefix("jail.")
                .tempdir()
                .unwrap()
        );

        // FIXME: Don't hardcode path to executable
        // FIXME: Currently, the executable is not statically linked
        std::fs::copy(
            "/home/me/dev/jail/target/debug/asynts-example",
            self.directory.as_ref().unwrap().path().join("application")
        ).unwrap();
    }
}

fn main() {
    // Many operations require the 'CAP_SYS_ADMIN' capability, this check is
    // not too far off.
    assert!(util::is_root_user());

    let mut service = Service::new();
    service.launch();

    println!("directory: {:?}", service.directory.as_ref().unwrap().path());

    print!("Press ENTER to continue...");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
