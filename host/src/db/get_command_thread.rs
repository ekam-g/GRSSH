
use std::thread;
use std::time::{Duration, Instant};

use crate::command::logging::Log;
use crate::{db};
use crate::command::special;

pub fn get_command() -> Result<String, String> {
    let mut wait_long = false;
    let sleep = Instant::now();
    let mut sleep_time: u16 = 2;
    loop {
        let check = check_command();
        if let Some(check) = check {
            return check;
        }
        if sleep.elapsed() > Duration::from_secs(360) && !wait_long {
            warn!("Server Entering Power Saver Mode\n");
            wait_long = true;
            sleep_time = 5000;
        }
        thread::sleep(Duration::from_millis(sleep_time as u64))
    }
}

pub fn check_command() -> Option<Result<String,String>> {
    let data = db::get();
    if let Ok(Some(good)) = data {
        if good.contains("&&") {
            let log = good.replace("&&", "");
            thread::spawn(move || {
                Log::write(log);
            });
            let view = good.trim();
            if special::end_check_or_sleep(view).is_some() {
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
            return Some(Ok(special::cd_command(good)));
        }
    }
    None
}