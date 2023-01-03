use std::fmt::Display;
use std::process::exit;
use std::thread;
use std::time::Duration;
use redis::{IntoConnectionInfo, RedisResult, ToRedisArgs};
use redis::{Client, Commands, Connection};
use crate::ENCRYPTION;
use crate::ram_var::HostData;

pub fn client() -> RedisResult<Connection> {
    try_client(HostData::get().redis_key.clone())
}
pub fn try_client<T : IntoConnectionInfo>(redis_key : T) -> RedisResult<Connection> {
    let redis = Client::open(redis_key)?;
    redis.get_connection()
}

pub fn encrypt(data: String) -> Option<String> {
    let mut return_data: Vec<String> = vec![];
    encrypted_id::init("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h61");
    for letter in data.into_bytes() {
        let pusb_val = encrypted_id::encrypt(letter as u64, ENCRYPTION.key);
        if let Ok(push) = pusb_val {
            return_data.push(push);
        } else {
            return None;
        }
    }
    Some(return_data.join("/"))
}

pub fn decrypt(data: String) -> Option<String> {
    if !data.contains('/') {
        return None;
    }
    let mut return_data: Vec<u8> = vec![];
    encrypted_id::init("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h61");
    for letter in data.split('/') {
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
    Some(String::from_utf8(return_data).unwrap())
}

pub struct Encrypt<'a> {
    pub key: &'a str,
}

fn where_send<T : Display, E : ToRedisArgs>(val: T, location : E) -> Option<RedisResult<bool>>{
    let send = encrypt(val.to_string());
    if let Some(send) = send {
        let client = client();
        return match client {
            Ok(mut connection) =>{
                Some(connection.set(location, send))
            }
            Err(e) => {
                Some(Err(e))
            }
        }
    }
    None
}

pub fn send<T: Display>(val: T) -> Option<RedisResult<bool>> {
    where_send(val, HostData::get().connect.clone())
}
pub fn get_path() -> RedisResult<String> {
    let mut  client = client()?;
    client.get(path())
}
pub fn get() -> RedisResult<Option<String>> {
    let data = client();
    match data {
        Ok(mut client)=> {
            let data = client.get(HostData::get().connect.clone());
            match data {
                Err(e)=> Err(e),
                Ok(data)=> {
                    Ok(decrypt(data))
                }
            }

        }
        Err(e) => Err(e),
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
