extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/child_main.c");

    cc::Build::new()
        .file("src/child_main.c")
        .compile("libjail.a");
    
    println!("cargo:rustc-link-lib=static=jail");
}
