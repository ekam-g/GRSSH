use std::{thread, time};

use crate::db::{send};
use crate::input::{get, y_n};
use crate::ram_var::HostData;

pub fn client_main() {
    loop {
        let (output ,path)  = wait_for_new();
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
    let mut _time: i8 = 0;
    let mut dead_server: bool = false;
    loop {
        let data = crate::db::get();
        let path_data = crate::db::get_path();
        if let (Ok(command), Ok(path )) = (data, path_data) {
            if command.contains("**") {
                return (command.replace("**", ""), path);
            }
        }
        if _time == 60 {
            if dead_server {
                println!("host pc might be dead or not responding, waiting.........")
            } else if y_n("Command kill?(y or n)") {
                let status = send(&"&&kill".to_owned());
                match status {
                    Ok(_) => {
                        println!("killing......");
                        dead_server = true;
                        _time = 0;
                    }
                    Err(oh_no) => {
                        println!("{oh_no}")
                    }
                }
            }
            _time = 0;
        }
        _time += 1;
        thread::sleep(time::Duration::from_millis(10));
    }
}
