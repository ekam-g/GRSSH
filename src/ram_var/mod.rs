use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};
use txt_writer;
pub struct HostData {
    pub data: String,
    pub kill_thread: bool,
    pub redis_key: String,
    pub location: String,
    pub last_working_location: String,
}

pub static HOST_VAR: Lazy<Mutex<HostData>> = Lazy::new(|| {
    let _data = txt_writer::ReadData {}
        .read_one(crate::LOCATION_TO_REDIS_KEY)
        .unwrap_or(
            {
                txt_writer::WriteData{}.replace("Add key here","redis_key.txt")
                    .expect("please allow writing permissions");
                panic!("Please add redis Key");
            }
        )
        .trim()
        .to_owned();
    Mutex::new(HostData {
        data: String::new(),
        kill_thread: false,
        redis_key: _data,
        location: String::new(),
        last_working_location: String::new(),
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
            dbg!("problem");
        }
    }
}
