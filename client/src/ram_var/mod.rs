use std::process::exit;
use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

 pub struct HostData {
    pub connect: String,
    pub redis_key: String,

}

pub static HOST_VAR: Lazy<Mutex<HostData>> = Lazy::new(|| {
    let _data = {
        let try_read = txt_writer::ReadData {}
            .read_one(crate::config::LOCATION_TO_REDIS_KEY);
        match try_read {
            Ok(data) => data.trim().to_owned(),
            Err(error_data) => {
                let _ = txt_writer::WriteData{}.replace("Add key here","redis_key.txt");
                println!("failed to read redis key, please set it or change permissions.\n{error_data}");
                exit(0);
            }
        }
    };
    Mutex::new(HostData {
        connect:  String::new(),
        redis_key: _data,
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
