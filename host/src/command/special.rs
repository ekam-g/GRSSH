use std::process::exit;
use std::thread;
use std::time::Duration;
use crate::db;

pub fn cd_command(good: String) -> String {
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
    warn!("changing location to {}\n", &data.location.join("/"));
    "ls".to_owned()
}

pub fn end_check_or_sleep(data: &str) -> Option<()> {
    if data == "&&quit" {
        loop {
            if db::send("**server shutting down").is_some() {
                warn!("server shutting down\n");
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
                    if db::send("$$server sleeping").is_some() {
                        warn!("server sleeping for {sleep_time} min\n");
                        thread::sleep(Duration::from_secs(sleep_time * 60));
                        warn!("Server Back Online\n");
                        return Some(());
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }
    None
}
