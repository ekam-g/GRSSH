use std::{thread, time};

use crate::command::exc;
use crate::db::get_command_thread::get_command;
use crate::db::send;
use crate::ram_var::HOST_VAR;


pub fn host_main() {
    loop {
        let data = get_command();
        let thread_worker = thread::spawn(move || {
            let result = exc(data);
            
        });
        let result = exc(data);
        send(&format!("**{}", result)).unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}
