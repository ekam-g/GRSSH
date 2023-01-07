use std::fmt::Display;
use std::process::exit;
use std::thread;
use std::time::Duration;
use redis::{IntoConnectionInfo, RedisResult, ToRedisArgs};
use redis::{Client, Commands, Connection};
use crate::config::ENCRYPTION;
use crate::ram_var::HostData;

pub fn client() -> RedisResult<Connection> {
    try_client(HostData::get().redis_key.clone())
}

pub fn try_client<T: IntoConnectionInfo>(redis_key: T) -> RedisResult<Connection> {
    let redis = Client::open(redis_key)?;
    redis.get_connection()
}

pub fn encrypt(data: String) -> Option<String> {
    if !ENCRYPTION.encryption_on {
        return Some(data);
    }
    let mut return_data: Vec<String> = vec![];
    encrypted_id::init("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h61");
    for letter in data.into_bytes() {
        let pub_val = encrypted_id::encrypt(letter as u64, ENCRYPTION.key);
        if let Ok(push) = pub_val {
            return_data.push(push);
        } else {
            return None;
        }
    }
    Some(return_data.join("oifago"))
}

pub fn decrypt(data: String) -> Option<String> {
    if !ENCRYPTION.encryption_on {
        return Some(data);
    }
    let mut return_data: Vec<u8> = vec![];
    encrypted_id::init("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h61");
    for letter in data.split("oifago") {
        let id = encrypted_id::decrypt(letter, ENCRYPTION.key);
        if let Ok(id) = id {
            if let Ok(id) = id.to_string().parse() {
                return_data.push(id)
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    match String::from_utf8(return_data){
        Ok(data) => {
            Some(data)
        }
        Err(_) => {
            None
        }
    }
}

pub struct Encrypt<'a> {
    pub key: &'a str,
    pub encryption_on: bool,
}

fn where_send<T: Display, E: ToRedisArgs>(val: T, location: E) -> Option<RedisResult<bool>> {
    let send = encrypt(val.to_string());
    if let Some(send) = send {
        let client = client();
        return match client {
            Ok(mut connection) => {
                Some(connection.set(location, send))
            }
            Err(e) => {
                Some(Err(e))
            }
        };
    }
    None
}

pub fn send<T: Display>(val: T) -> Option<RedisResult<bool>> {
    let data = HostData::get();
    let location = data.connect.clone();
    drop(data);
    where_send(val, location)
}

pub fn get_path() -> RedisResult<Option<String>> {
    match where_get(path()) {
        Err(e) => Err(e),
        Ok(data) => {
            Ok(decrypt(data))
        }
    }
}

fn where_get<T: ToRedisArgs>(val: T) -> RedisResult<String> {
    let mut client = client()?;
    client.get(val)
}

pub fn get() -> RedisResult<Option<String>> {
    let data = HostData::get();
    let location = data.connect.clone();
    drop(data);
    match where_get(location) {
        Err(e) => Err(e),
        Ok(data) => {
            Ok(decrypt(data))
        }
    }
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
                        if servers.is_empty() {
                            no_server_exit();
                        }
                        let mut working_servers: Vec<String> = vec![];
                        for server in servers.clone() {
                            if let Ok(data) = where_get(&server) {
                                if decrypt(data).is_some() {
                                    working_servers.push(server)
                                }
                            }
                        }
                        if working_servers.is_empty() {
                            no_server_exit()
                        }
                        println!("The Servers on: {}\n\nconnect to who?", working_servers.join(", "));
                        loop {
                            let user_input = crate::input::get().trim().to_owned();
                            if working_servers.contains(&user_input) {
                                HostData::get().connect = user_input;
                                return;
                            }
                            println!("bad name please try again")
                        }
                    }
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

fn no_server_exit() {
    println!("no server found or your encryption key is not correct, please fix it and try again.");
    exit(1)
}