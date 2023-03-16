use std::{thread};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::command::exc;
use crate::db;
use crate::db::get_command_thread::{get_command};
use crate::db::{format_path, lock_str, send, send_path};
use crate::ram_var::{HostData, ERRORS};



pub fn host_main() {
    info!("Global SSH Server Is Successfully Turned On\n");
    loop {
        info!("Waiting for Command.....\n");
        let data = get_command();
        let send_data: String;
        match data {
            Ok(data) => {
                info!("running command {}\n", &data);
                let mut cool = Arc::new(Mutex::new(String::new()));
                let mut pub_data = Arc::clone(&cool);
                let thread_worker = thread::spawn(move || {
                    let result = exc(data);
                    let mut locked = lock_str(&mut pub_data);
                    *locked = result;
                    info!("Finished Command, Data is\n {}\n", locked);
                });
                let time_passed = Instant::now();
                let result: String = loop {
                    if thread_worker.is_finished() {
                        let locked = lock_str(&mut cool);
                        break locked.to_string();
                    }
                    if time_passed.elapsed() > Duration::from_secs(7) {
                        if let Ok(Some(kill)) = db::get() {
                            if kill == *"&&kill" {
                                warn!("process killed\n");
                                break "killed".to_owned();
                            }
                        }
                    }
                };
                send_data = result;
            }
            Err(message) => {
                send_data = message;
            }
        }
        wait_send_data(send_data)
    }
}

fn wait_send_data(result: String) {
    loop {
        let pub_var = HostData::get();
        let path = pub_var.last_working_location.clone();
        drop(pub_var);
        if send_path(format_path(path)).is_some() && send(format!("**{result}", )).is_some() {
            return;
        }
        error!("{}", ERRORS.redis_send_error);
        thread::sleep(Duration::from_secs(1));
    }
}