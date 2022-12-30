use std::{thread};
use std::process::exit;
use std::time::{Duration, Instant};

use crate::db::send;
use crate::input::{get, y_n};
use crate::ram_var::HostData;

pub fn client_main() {
    loop {
        let (output, path) = wait_for_new();
        println!("\n{}\n{} in {} -->", output, HostData::get().connect, path);
        let user_input = get();
        let error = {
            if user_input.contains("cd") {
                send(&format!("%%{user_input}"))
            } else {
                send(&format!("&&{user_input}"))
            }
        };
        match error {
            Ok(_) => {}
            Err(_) => {
                println!("error when connecting to redis")
            }
        }
    }
}

fn wait_for_new() -> (String, String) {
    let mut tried: i8 = 0;
    let mut dead_server: bool = false;
    let mut time_secs = Instant::now();
    loop {
        let data = crate::db::get();
        let path_data = crate::db::get_path();
        if let (Ok(command), Ok(path)) = (data, path_data) {
            if command.contains("**") {
                return (command.replace("**", ""), path);
            }
            if command.contains("$$") {
                if !dead_server {
                    println!("Server is Sleeping");
                    dead_server = true;
                }
                time_secs = Instant::now();
            }
        }
        if time_secs.elapsed() > Duration::from_secs(8) {
            if dead_server{
                println!("host pc might be dead or not responding, waiting: {tried}.........\n");
                tried += 1;
                if tried > 65 {
                    println!("no response received from host, shutting off");
                    exit(1);
                }
            } else if y_n("Command kill?(y or n)") {
                let status = send(&"&&kill".to_owned());
                match status {
                    Ok(_) => {
                        println!("killing......");
                        dead_server = true;
                    }
                    Err(oh_no) => {
                        println!("{oh_no}")
                    }
                }
            }
            time_secs = Instant::now();
        }
        thread::sleep(Duration::from_millis(2));
    }
}
