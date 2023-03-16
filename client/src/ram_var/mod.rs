use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

 pub struct HostData {
    pub connect: String,

}

pub static HOST_VAR: Lazy<Mutex<HostData>> = Lazy::new(|| {
    Mutex::new(HostData {
        connect:  String::new(),
    })
});

impl HostData {
    // this may cause errors
    pub fn get() -> MutexGuard<'static, Self> {
        loop {
            let check = HOST_VAR.try_lock();
            if let Ok(data) = check {
                return data;
            }
            dbg!("waiting on lock.....");
        }
    }
}
