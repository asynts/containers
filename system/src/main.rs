// Will be run in isolated environment.

fn main() {
    println!("I am executing in an isolated environment!");

    println!("Here are all the files I have access to:");
    for entry in walkdir::WalkDir::new("/") {
        println!("  {}", entry.unwrap().path().display());
    }
}
