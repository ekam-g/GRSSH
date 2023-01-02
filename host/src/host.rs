use std::thread;
use std::time::Duration;

use crate::command::exc;
use crate::db::{format_path, send, send_path};
use crate::db::get_command_thread::{check_command, get_command};
use crate::ram_var::HostData;

pub fn host_main() {
    loop {
        reset();
        let data = get_command();
        let _ = send("read");
        let send_data: String;
        match data {
            Ok(data) => {
                println!("running command {}", &data);
                let thread_worker = thread::spawn(move || {
                    let result = exc(data);
                    let mut pub_data = HostData::get();
                    pub_data.data = result;
                    println!("finished command\n {}", &pub_data.data);
                });
                let result: String = loop {
                    if thread_worker.is_finished() {
                        break HostData::get().data.clone();
                    }
                    if let Some(Ok(kill)) = check_command() {
                        if kill == *"kill" {
                            println!("process killed");
                            break "killed".to_owned();
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
        let path = HostData::get().last_working_location.clone();
        if send_path(format_path(path)).is_some() && send(format!("**{result}", )).is_some() {
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
