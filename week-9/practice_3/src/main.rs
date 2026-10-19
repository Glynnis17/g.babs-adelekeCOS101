use std::io::Write;
use std::fs;
fn main() {
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n"
        .as_bytes()).expect("create failed");


    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}
