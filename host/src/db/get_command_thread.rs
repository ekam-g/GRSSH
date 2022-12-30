use std::process::exit;
use std::thread;
use std::time::{Duration, Instant};

use crate::command::logging::Log;
use crate::db;

pub fn get_command() -> Result<String, String> {
    let sleep = Instant::now();
    let mut sleep_time: u16 = 2;
    loop {
        let check = check_command();
        if let Some(check) = check {
            return check;
        }
        if sleep.elapsed() > Duration::from_secs(360) {
            sleep_time = 5000;
        }
        thread::sleep(Duration::from_millis(sleep_time as u64))
    }
}

pub fn check_command() -> Option<Result<String,String>> {
    let data = db::get();
    if let Ok(good) = data {
        if good.contains("&&") {
            let log = good.replace("&&", "");
            thread::spawn(move || {
                Log::write(log);
            });
            let view = good.trim();
            if end_check_or_sleep(view).is_some() {
                return Some(Err("Server back Online".to_owned()));
            }
            if let Some(data) = Log::read(view){
                return Some(Err(data))
            }
            return Some(Ok(good.replace("&&", "")));
        } else if good.contains("%%") {
            let log = good.replace("%%", "");
            thread::spawn(move || {
                Log::write(log);
            });
            return Some(Ok(cd_command(good)));
        }
    }
    None
}


fn cd_command(good: String) -> String {
    let mut data = crate::ram_var::HostData::get();
    if good.contains("cd -") {
        let remove_location = data.location.len() - 1;
        if data.location.get(remove_location).is_some() {
            let remove = &data.location[remove_location].clone();
            data.location.retain(|x| x != remove);
        }
    } else if good.contains("cd ~") {
        data.location = vec![];
        for path in good.split('/') {
            data.location.push(path.to_owned());
        }
    } else if good.replace("%%", "").trim() == "cd" {
        data.location = vec![];
    } else {
        data.location.push(good.replace("%%", "").replace("cd", "").trim().to_owned());
    }
    data.last_working_location.retain(|x| !x.is_empty());
    data.location.retain(|x| !x.is_empty());
    println!("changing location to {}", &data.location.join("/"));
    "ls".to_owned()
}

fn end_check_or_sleep(data: &str) -> Option<()> {
    if data == "&&quit" {
        loop {
            if db::send("**server shutting down").is_ok() {
                println!("server shutting down");
                exit(1);
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
    if data.contains("command-sleep") {
        let split = data.split(' ').nth(1);
        if let Some(sleep_amount_unchecked) = split {
            if let Ok(sleep) = sleep_amount_unchecked.parse() {
                let sleep_time: u64 = sleep;
                loop {
                    if db::send("$$server sleeping").is_ok() {
                        println!("server sleeping for {sleep_time} min");
                        thread::sleep(Duration::from_secs(sleep_time * 60));
                        return Some(());
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }
    None
}
