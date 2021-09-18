extern crate walkdir;
extern crate nix;
extern crate tempfile;

mod util {
    pub fn is_statically_linked() -> bool {
        cfg!(target_feature = "crt-static")
    }
}

fn _test_filesystem_access() {
    println!("All the files:");
    for entry in walkdir::WalkDir::new("/") {
        let entry = entry.unwrap();
        println!("  {}", entry.path().display());
    }
}

fn _test_remount_escape() {
    std::fs::create_dir("/foo").unwrap();
    nix::mount::mount::<str, str, str, str>(Some("/"), "/foo", None, nix::mount::MsFlags::MS_BIND, None).unwrap();
}

fn _test_unmount_root() {
    nix::mount::umount("/").unwrap();
}

fn _test_bind_mount() {
    std::fs::create_dir("/foo").unwrap();
    std::fs::create_dir("/bar").unwrap();

    nix::mount::mount::<str, str, str, str>(Some("/foo"), "/bar", None, nix::mount::MsFlags::MS_BIND, None).unwrap();

    nix::mount::umount("/bar").unwrap();
}

fn _test_tempfile() {
    let tempdir = tempfile::tempdir().unwrap();
    println!("tempdir={}", tempdir.path().display());
}

fn main() {
    assert!(util::is_statically_linked());
}
