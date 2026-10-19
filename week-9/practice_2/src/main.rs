use std::io::Write;
use std::io::Read;

fn main() {
    let greet = "Welcome";


    let mut file = std::fs::File::create("welcome_message.txt").expect("create failed");
    file.write(greet.as_bytes()).expect("write failed");
    

    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    print!("{}", contents);
}
