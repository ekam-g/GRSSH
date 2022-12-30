use std::process::exit;
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
                let _ = txt_writer::WriteData{}.replace("Add key here","redis_key.txt");
                println!("failed to read redis key, please set it or change permissions.\n{error_data}");
                exit(1);
            }
        }
    };
    let client = {
        match make_client(_data.clone()) {
            Ok(good) => good,
            Err(e) =>{
                println!("failed trying to make redis client make sure you wifi set and your redis key is good\n{e}");
                exit(1);
            }
        }
    };
    let location  = {
        let mut _location = crate::db::get_path(_data);
        _location.retain( | x| ! x.is_empty());
        _location
    };
    Mutex::new(HostData {
        data: String::new(),
        kill_thread: false,
        location: location.clone(),
        last_working_location: location,
        client
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

