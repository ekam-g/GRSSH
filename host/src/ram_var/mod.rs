use std::process::exit;
use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};
use redis::Client;
use crate::config::REDIS_KEY;
use crate::db::make_client;

pub struct HostData {
    pub kill_thread: bool,
    pub location: Vec<String>,
    pub last_working_location: Vec<String>,
}

pub static HOST_VAR: Lazy<Mutex<HostData>> = Lazy::new(|| {
    let location  = {
        let mut _location = crate::db::get_path(REDIS_KEY);
        _location.retain( | x| ! x.is_empty());
        _location
    };
    Mutex::new(HostData {
        kill_thread: false,
        location: location.clone(),
        last_working_location: location,
    })
});

pub static REDIS_CLIENT: Lazy<Mutex<Client>> = Lazy::new(|| {
    Mutex::new(
        match make_client(REDIS_KEY) {
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
            error!("dead lock problem");
        }
    }
    pub fn get_client() -> Client {
        loop {
            let check = REDIS_CLIENT.try_lock();
            if let Ok(data) = check {
                return data.clone();
            }
            error!("dead lock problem");
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