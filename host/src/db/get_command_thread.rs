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
            return cd_command(good);
        }
    }
    None
}

fn cd_command(good : String) -> Option<String> {
    let mut data = crate::ram_var::HostData::get();
    if good.contains("cd -") {
        let remove_location  = data.location.len() - 1;
        if data.location.get(remove_location).is_some() {
            let remove = &data.location[remove_location].clone();
            data.location.retain(|x| x != remove);
        }
    }
    else if good.contains("cd ~")  {
        data.location = vec![];
        for path in good.split('/') {
            data.location.push(path.to_owned());
        }
    }
    else if good.replace("%%", "").trim() == "cd" {
        data.location = vec![];
    }
    else {
        data.location.push(good.replace("%%", "").replace("cd", "").trim().to_owned());
    }
    data.last_working_location.retain(|x| !x.is_empty());
    data.location.retain(|x| !x.is_empty());
    println!("changing location to {}", &data.location.join("/"));
    Some("ls".to_owned())
}

fn end_check(data: &str) {
    if data == "&&quit" {
        loop {
            if db::send("**server shutting down").is_ok() {
                println!("server shutting down");
                exit(1);
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}