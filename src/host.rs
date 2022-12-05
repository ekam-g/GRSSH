use std::f32::consts::E;
use std::{thread, time};

use crate::command::exc;
use crate::db::get_command_thread::{check_command, get_command};
use crate::db::send;
use crate::ram_var::HostData;

pub fn host_main() {
    loop {
        reset();
        let data = get_command();
        let thread_worker = thread::spawn(move || {
            let result = exc(data);
            let mut pub_data = HostData::get();
            pub_data.done = true;
            pub_data.data = result;
        });
        let result: String = loop {
            let pub_data = HostData::get();
            if pub_data.done == true {
                break pub_data.data.clone();
            }
            if let Some(kill) = check_command() {
                if kill == *"kill" {
                    "kill".to_owned()
                }
            }
        };
        send(&format!("**{}", result)).unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn reset() {
    let mut data = HostData::get();
    data.kill_thread = false;
    data.done = false;
    data.data = String::new();
}
