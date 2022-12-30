use std::process::exit;
use std::thread;
use std::time::Duration;
use redis::{IntoConnectionInfo, RedisResult};
use redis::{Client, Commands, Connection};
use crate::ram_var::HostData;

pub fn client() -> RedisResult<Connection> {
    try_client(HostData::get().redis_key.clone())
}
pub fn try_client<T : IntoConnectionInfo>(redis_key : T) -> RedisResult<Connection> {
    let redis = Client::open(redis_key)?;
    redis.get_connection()
}
pub fn send(val: &String) -> RedisResult<bool> {
    let mut client = client()?;
    client.set_ex(HostData::get().connect.clone(), val, 360)
}

pub fn get() -> RedisResult<String> {
    let mut client = client()?;
    client.get(HostData::get().connect.clone())
}
pub fn get_path() -> RedisResult<String> {
    let mut  client = client()?;
    client.get(path())
}
pub fn path() -> String {
    format!("{}location", HostData::get().connect.clone())
}
pub fn who() {
    let mut error_amount: i8 = 0;
    while error_amount < 120 {
        let error;
        let client = client();
        match client {
            Ok(mut client) => {
                let servers: RedisResult<Vec<String>> = client.keys("*");
                match servers {
                    Ok(mut servers) => {
                        servers.retain(|x| !x.contains("location"));
                        println!("The Servers on: {}\n\nconnect to who?", servers.join(", "));
                        HostData::get().connect = crate::input::get().trim().to_owned();
                        return;
                    },
                    Err(e) => {
                        error = e;
                    }
                }
            }
            Err(e) => {
                error = e;
            }
        }
        thread::sleep(Duration::from_millis(10));
        error_amount += 1;
        println!("error when connecting to redis, retrying {error_amount}\n{error}");
    }
    println!("exiting, please read error and try to check wifi, redis server, and redis key.");
    exit(0);
}