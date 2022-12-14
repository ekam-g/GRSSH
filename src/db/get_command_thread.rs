use std::{thread, time};

pub fn get_command() -> String {
    loop {
        if let Some(data) = check_command() {
            return data;
        }
        thread::sleep(time::Duration::from_millis(10))
    }
}

pub fn check_command() -> Option<String> {
    let data = crate::db::get();
    if let Ok(good) = data {
        if good != "read".to_owned() && good.contains("&&") {
            let send = crate::db::send(&"read".to_owned());
            if let Ok(_) = send {
                return Some(good.replace("&&", ""));
            }
        } else if good.contains("%%") {
            let mut data = crate::ram_var::HostData::get();
            data.location = good.replace("%%", "").replace("cd", "").trim().to_owned();
            return Some("ls".to_owned());
        }
    }
    None
}
