use once_cell::sync::Lazy;
use std::sync::{Mutex};

pub struct Host {
    pub data: String,
    pub kill_thread: bool,
}

pub static HOST_VAR: Lazy<Mutex<Host>> = Lazy::new(|| {
    Mutex::new(Host {
        data: String::new(),
        kill_thread: false,
    })
});

impl Host {
    // this may cause errors
    pub fn get() -> Self {
        loop {
            let check = HOST_VAR.lock();
            if let Ok(data) = check {
                return Self {
                    data : data.data,
                    kill_thread : data.kill_thread
                };
            }
        }
    }
}
