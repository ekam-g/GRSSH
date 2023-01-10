use std::{thread};
use std::process::exit;
use std::time::{Duration, Instant};

use crate::db::{delete_key, send};
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
            Some(Ok(_)) => {}
            None => {
                println!("error when trying to encrypt data, please check source code and report this")
            }
            Some(Err(_)) => {
                println!("error when connecting to redis")
            }
        }
    }
}

fn wait_for_new() -> (String, String) {
    let mut tried: i8 = 0;
    let mut dead_server: bool = false;
    let mut method: bool = false;
    let mut time_secs = Instant::now();
    loop {
        let data = crate::db::get();
        let path_data = crate::db::get_path();
        if let (Ok(Some(command)), Ok(Some(path))) = (&data, &path_data) {
            if command.contains("**") {
                return (command.replace("**", ""), path.to_owned());
            }
            if command.contains("$$") {
                if !dead_server {
                    println!("Server is Sleeping");
                    dead_server = true;
                }
                time_secs = Instant::now();
            }
        } else if let Ok(None) = data{
            return ("encryption error".to_owned(), "error".to_owned());
        } else  if let Ok(None) = path_data {
            return ("encryption error".to_owned(), "error".to_owned());
        }
        if time_secs.elapsed() > Duration::from_secs(8) {
            if dead_server{
                println!("host pc might be dead or not responding, waiting: {tried}.........\n");
                let _ = delete_key();
                if method {
                    method = false;
                    let _ = send("&&kill");
                } else {
                    method = true;
                    let _ = send("&&ls");
                }
                tried += 1;
                if tried > 65 {
                    println!("no response received from host, shutting off");
                    exit(1);
                }
            } else if y_n("Command kill?(y or n)") {
                let status = send(&"&&kill".to_owned());
                match status {
                    Some(Ok(_)) => {
                        println!("killing......");
                        dead_server = true;
                    }
                    Some(Err(oh_no)) => {
                        println!("Redis send data error\n{oh_no}")
                    }
                    None => {
                        println!("encryption error occurred, please check your key and try again")
                    }
                }
            }
            time_secs = Instant::now();
        }
        thread::sleep(Duration::from_millis(2));
    }
}
