use std::io;

pub fn get() -> String {
    let mut return_data = String::new();
    io::stdin()
        .read_line(&mut return_data)
        .expect("Failed to read input");
    return_data
}