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
    redis_key: String,
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
                exit(1);
            }
        }
    };
    let location  = {
        let mut _location = crate::db::get_path(_data.clone());
        _location.retain( | x| ! x.is_empty());
        _location
    };
    Mutex::new(HostData {
        data: String::new(),
        kill_thread: false,
        location: location.clone(),
        last_working_location: location,
        redis_key : _data,
    })
});

pub static REDIS_CLIENT: Lazy<Mutex<Client>> = Lazy::new(|| {
    Mutex::new(
        match make_client(HostData::get().redis_key.clone()) {
            Ok(good) => good,
            Err(e) =>{
                println!("failed trying to make redis client make sure you wifi set and your redis key is good\n{e}");
                exit(1);
            }
        }
    )
});

impl HostData {
    // this may cause errors
    pub fn get() -> MutexGuard<'static, Self> {
        loop {
            let check = HOST_VAR.try_lock();
            if let Ok(data) = check {
                return data;
            }
            dbg!("dead lock problem");
        }
    }
    pub fn get_client() -> Client {
        loop {
            let check = REDIS_CLIENT.try_lock();
            if let Ok(data) = check {
                return data.clone();
            }
            dbg!("dead lock problem");
        }
    }
}

pub struct Errors<'a> {
    pub redis_send_error : &'a str,
    pub redis_get_error : &'a str,
}

pub const ERRORS: Errors = Errors  {
    redis_get_error : "Failed to Get Data From Redis, Retrying.......\n",
    redis_send_error: "Failed to Send Data to Redis, Retrying......\n",
};