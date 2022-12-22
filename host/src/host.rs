use std::{thread};
use std::time::Duration;

use crate::command::exc;
use crate::db::get_command_thread::{check_command, get_command};
use crate::db::{send, send_path};
use crate::ram_var::HostData;

pub fn host_main() {
    loop {
        reset();
        let data = get_command();
        println!("running command {}" , &data);
        let thread_worker = thread::spawn(move || {
            let result = exc(data);
            let mut pub_data = HostData::get();
            pub_data.data = result;
            println!("finished command\n {}" , &pub_data.data);
        });
        let result: String = loop {
            let pub_data = HostData::get();
            if thread_worker.is_finished() {
                break pub_data.data.clone();
            }
            drop(pub_data);
            if let Some(kill) = check_command() {
                if kill == *"kill" {
                    println!("process killed");
                    break "killed".to_owned();
                }
            }
        };
        wait_send_data(result);
        thread::sleep(Duration::from_millis(10));
    }
}

fn wait_send_data(result : String) {
    loop {
        let path = HostData::get().last_working_location.clone();
        if send_path(path.join("/")).is_ok() && send(&format!("**{}", result)).is_ok() {
            return;
        }
        println!("problem when sending data to redis. Retrying.......");
        thread::sleep(Duration::from_secs(1));
    }
}

fn reset() {
    let mut data = HostData::get();
    data.data = String::new();
}
