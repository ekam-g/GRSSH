use std::io;
use std::process::Command;

fn main() {
    loop {
        println!("{}",command(get()));
    }
}

// fn new() -> Command {
//     Command::new("bash")
// }

pub fn get() -> String {
    let mut return_data = String::new();
    io::stdin()
        .read_line(&mut return_data)
        .expect("Failed to read input");
    return_data
}
pub fn command(what: String) -> String {
    let (first, rest) = {
        let (mut first, mut rest) = ("", vec![]);
        let first_done = false;
        for word in what.split_whitespace() {
            if !first_done {
                first = word;
            } else {
                rest.push(word)
            }
        }
        (first, rest)
    };
    let success = Command::new(first).args(rest).output();

    match success {
        Ok(good) => {
            String::from_utf8(good.stdout).unwrap()
        }
        Err(_) => {
            let val = Command::new("bash")
                .args(["-c", "./global_ssh"])
                .args(what.split_whitespace())
                .output()
                .unwrap()
                .stdout;

            String::from_utf8(val).unwrap()
        }
    }
}
