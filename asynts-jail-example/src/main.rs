extern crate walkdir;

mod util {
    pub fn is_statically_linked() -> bool {
        cfg!(target_feature = "crt-static")
    }
}

fn test_filesystem_access() {
    println!("All the files:");
    for entry in walkdir::WalkDir::new("/") {
        let entry = entry.unwrap();
        println!("  {}", entry.path().display());
    }
}

// FIXME: Try to remount '/' to somewhere else.

fn main() {
    assert!(util::is_statically_linked());

    test_filesystem_access();
}
