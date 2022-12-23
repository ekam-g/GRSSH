use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};
use redis::Client;
use txt_writer;
use crate::db::make_client;

pub struct HostData {
    pub data: String,
    pub kill_thread: bool,
    pub location: Vec<String>,
    pub last_working_location: Vec<String>,
    pub client : Client,
}

pub static HOST_VAR: Lazy<Mutex<HostData>> = Lazy::new(|| {
    let _data = {
        let try_read = txt_writer::ReadData {}
            .read_one(crate::LOCATION_TO_REDIS_KEY);
        match try_read {
            Ok(data) => data.trim().to_owned(),
            Err(error_data) => {
                txt_writer::WriteData{}.replace("Add key here","redis_key.txt")
                    .expect("please allow writing permissions");
                panic!("failed to read redis key, please set it or change permissions.\n{}",error_data );
            }
        }
    };
    let mut _location = crate::db::get_path(_data.clone());
    _location.retain(|x| !x.is_empty());
    Mutex::new(HostData {
        data: String::new(),
        kill_thread: false,
        location: _location.clone(),
        last_working_location: _location,
        client : make_client(_data).expect("Please Set your redis key properly")
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
            dbg!("dead lock problem");
        }
    }
}

