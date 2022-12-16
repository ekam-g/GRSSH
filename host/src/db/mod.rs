pub mod get_command_thread;

use std::fs;
use redis::{IntoConnectionInfo, RedisResult};
use redis::{Client, Commands, Connection};

//TODO refactor soon

pub fn client() -> RedisResult<Connection> {
    dbg!("Somthdsgas");

    try_client(crate::ram_var::HostData::get().redis_key.clone())
}

pub fn try_client<T: IntoConnectionInfo>(redis_key: T) -> RedisResult<Connection> {
    let redis = Client::open(redis_key)?;
    redis.get_connection()
}

pub fn send(val: &String) -> RedisResult<bool> {
    dbg!("Somthdsgas");

    let mut client = client()?;
    dbg!("Somthdsgas");

    client.set(crate::NAME, val)
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
                Ok(data) => {
                    let check: String = data;
                    if fs::read_dir(check.clone()).is_ok() {
                        return check
                    }
                    println!("unable to find old path, please cd into home directory");
                    let _ : RedisResult<bool> = good_client.set(crate::NAME,"**unable to find old path, please cd into home directory");
                    return String::new();
                },
                Err(_) => {
                    err += 1;
                    if err == 30 {
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
