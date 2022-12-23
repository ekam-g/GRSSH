use std::{fmt::Display, io};

pub fn get() -> String {
    let mut return_data = String::new();
    io::stdin()
        .read_line(&mut return_data)
        .expect("Failed to read input");
    println!("---------------------------------------------------------");
    return_data
}

pub fn y_n(message: impl Display) -> bool {
    println!("{message}");
    loop {
        let input = get();
        match input.trim() {
            "y" => {
                return true;
            }
            "n" => {
                return false;
            }
            _ => {
                println!("please respond with y or n")
            }
        }
    }
}
