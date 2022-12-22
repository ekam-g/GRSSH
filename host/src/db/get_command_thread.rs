use std::{thread};
use std::process::exit;
use std::time::Duration;
use crate::db;

pub fn get_command() -> String {
    loop {
        if let Some(data) = check_command() {
            return data;
        }
        thread::sleep(Duration::from_millis(10))
    }
}

pub fn check_command() -> Option<String> {
    let data = db::get();
    if let Ok(good) = data {
        if good.contains("&&") {
            end_check(good.trim());
            return Some(good.replace("&&", ""));
        } else if good.contains("%%") {
            let mut data = crate::ram_var::HostData::get();
            if good.contains("cd -") {
                //Todo fix bug here
                let del = data.location.len() - 1;
                data.location.remove(del);
            } else {
                data.location.push(good.replace("%%", "").replace("cd", "").trim().to_owned());
            }
            println!("changing location to {}", &data.location.join(""));
            return Some("ls".to_owned());
        }
    }
    None
}

fn end_check(data : &str){
    if data == "&&quit" {
        loop {
            if db::send(&"**server shutting down".to_owned()).is_ok() {
                println!("server shutting down");
                exit(1);
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}