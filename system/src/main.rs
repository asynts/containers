// Runs in jail.

fn main() {
    println!("I am executing in an isolated environment!");

    println!("Here are all the files I have access to:");
    for entry in walkdir::WalkDir::new("/") {
        println!("  {}", entry.unwrap().path().display());
    }

    // FIXME: List mounts; how?
}
