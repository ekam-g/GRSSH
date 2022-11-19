use std::io;
use std::process::Command;

fn main() {
    let result = new()
        .args(["-c","./global_ssh",])
        .output().
        unwrap()
        .stdout;
    println!("{}", String::from_utf8(result).unwrap());
}

fn new() -> Command {
    Command::new("bash")
}

pub fn get() -> String {
    let mut return_data = String::new();
    io::stdin()
        .read_line(&mut return_data)
        .expect("Failed to read input");
    return_data
}
