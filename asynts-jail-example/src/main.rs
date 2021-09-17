mod util {
    pub fn is_statically_linked() -> bool {
        cfg!(target_feature = "crt-static")
    }
}

fn main() {
    assert!(util::is_statically_linked());

    println!("Hello, world!");
}
