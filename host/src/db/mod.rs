pub mod get_command_thread;

use std::{thread, time};
use std::time::Duration;
use redis::{IntoConnectionInfo, RedisResult};
use redis::{Client, Commands, Connection};

//TODO refactor soon

pub fn client() -> RedisResult<Connection> {
    try_client(crate::ram_var::HostData::get().redis_key.clone())
}

pub fn try_client<T: IntoConnectionInfo>(redis_key: T) -> RedisResult<Connection> {
    let redis = Client::open(redis_key)?;
    redis.get_connection()
}

pub fn send(val: &String) -> RedisResult<bool> {
    let mut client = client()?;
    client.set(path(), val)
}

pub fn send_path(val: String) -> RedisResult<bool> {
    let mut client = client()?;
    client.set(path(), val)
}

pub fn path() -> String {
    format!("{}location", crate::NAME)
}

pub fn get_path(redis_location: String) -> String {
    let mut err: i8 = 0;
    loop {
        let client = try_client(redis_location.clone());
        if let Ok(mut good_client) = client {
            match good_client.get(path()) {
                Ok(data) => return data,
                Err(_) => {
                    err += 1;
                    if err == 120 {
                        thread::sleep(Duration::from_millis(10));
                        println!("unable to find old path, please cd into home directory");

                        let _ : RedisResult<bool> = good_client.set(crate::NAME,"**unable to find old path, please cd into home directory");
                        return  String::new()
                    }
                }
            }
        }
    }
}

pub fn get() -> RedisResult<String> {
    let mut client = client()?;
    client.get(crate::NAME)
}
