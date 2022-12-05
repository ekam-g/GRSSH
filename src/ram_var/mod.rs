use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

pub struct HostData {
    pub data: String,
    pub kill_thread: bool,
    pub done:  bool,
}

pub static HOST_VAR: Lazy<Mutex<HostData>> = Lazy::new(|| {
    Mutex::new(HostData {
        data: String::new(),
        kill_thread: false,
        done : false,
    })
});

impl HostData {
    // this may cause errors
    pub fn get() -> MutexGuard<'static, Self> {
        loop {
            let check = HOST_VAR.lock();
            if let Ok(data) = check {
                return data;
            }
        }
    }
}
