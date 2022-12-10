use std::{thread, time};

use crate::db::send;
use crate::input::{get, y_n};

pub fn client_main() {
    loop {
        println!("{}\n-->", wait_for_new());
        let user_input = get();
        let error = {
            if user_input.contains("cd") {
                send(&format!("%%{}", user_input))
            } else {
                send(&format!("&&{}", user_input))
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

fn wait_for_new() -> String {
    let mut time: i8 = 0;
    let mut dead_server: bool = false;
    loop {
        let data = crate::db::get();
        if let Ok(command) = data {
            if command.contains("**") {
                return command.replace("**", "");
            }
        }
        if time == 120 {
            if dead_server  {
                println!("host pc might be dead or not responding, waiting.........")
            } else if y_n("Command kill?(y or n)") {
                let status = send(&"&&kill".to_owned());
                match status {
                    Ok(_) => {
                        println!("killing......");
                        dead_server = true;
                        time = 0;
                    }
                    Err(oh_no) => {
                        println!("{}", oh_no)
                    }
                }
            }
            time = 0;
        }
        time += 1;
        thread::sleep(time::Duration::from_millis(10));
    }
}
