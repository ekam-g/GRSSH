use crate::db::send;
use crate::input::{get, y_n};
use std::{thread, time};

pub fn client_main() {
    loop {
        println!("{}\n-->", wait_for_new());
        let user_input = get();
        let error = send(&format!("&&{}", user_input));
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
    loop {
        let data = crate::db::get();
        if let Ok(command) = data {
            if command.contains("**") {
                return command.replace("**", "");
            }
        }
        if time == 120 {
            if y_n("kill?") {
                let status = send(&"&&kill".to_owned());
                match status {
                    Ok(_) => {
                        return "killed".to_owned();
                    },
                    Err(oh_no) => {
                        println!("{}", oh_no )
                    }
                }
            }
            time = 0;
        }
        time += 1;
        thread::sleep(time::Duration::from_millis(10));
    }
}
