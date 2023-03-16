use std::fmt::Display;
use std::fs;

use redis::{IntoConnectionInfo, RedisResult, ToRedisArgs};
use redis::{Client, Commands};

use crate::config::{ENCRYPTION, NAME};
use crate::ram_var::{ERRORS, HostData};

pub mod get_command_thread;
pub mod sentry_logging;
pub mod checks;


//TODO refactor soon

pub fn make_client<T: IntoConnectionInfo>(redis_key: T) -> RedisResult<Client> {
    Client::open(redis_key)
}


pub fn format_path(passed: Vec<String>) -> String {
    if cfg!(windows) {
        return format!("C:{}", passed.join("\\"));
    }
    format!("/{}", passed.join("/"))
}

pub fn send_path(val: String) -> Option<RedisResult<bool>> {
    where_send(val, path())
}

pub fn path() -> String {
    format!("{NAME}location")
}

pub fn get_path(redis_location: &str) -> Vec<String> {
    let mut return_val: Vec<String> = vec![];
    let mut err: i8 = 0;
    loop {
        let client = make_client(redis_location.clone());
        if let Ok(mut good_client) = client {
            match good_client.get(path()) {
                Ok(data) => {
                    let check = decrypt(data);
                    if let Some(check) = check {
                        match fs::read_dir(check.clone()) {
                            Ok(_) => {
                                for path in check.split('/') {
                                    return_val.push(path.to_owned());
                                }
                                return return_val;
                            },
                            Err(e) => {
                                error!("Failed to enter path {} because {}\n", check, e);
                            }
                        }
                    }
                    return set_unknown(good_client);
                }
                Err(e) => {
                    err += 1;
                    warn!("Failed when finding old path. Retrying {}.....\n{}\n", err, e);
                    if err == 30 {
                        return set_unknown(good_client);
                    }
                }
            }
        }
    }
}

fn set_unknown(mut good_client: Client) -> Vec<String> {
    warn!("unable to use old path, creating new path\n");
    let _: RedisResult<bool> = good_client.set(NAME, encrypt("**unable to find old path".to_owned()));
    let _: RedisResult<bool> = good_client.set(format!("{NAME}location"), encrypt("/".to_owned()));
    vec![]
}

pub fn get() -> RedisResult<Option<String>> {
    let client = HostData::get_client().get_connection()?.get(NAME);
    match client {
        Err(e) => {
            error!("{}\n{}\n", ERRORS.redis_get_error, e);
            Err(e)
        },
        Ok(data) => {
            Ok(decrypt(data))
        }
    }
}

pub fn encrypt(data: String) -> Option<String> {
    if !ENCRYPTION.encryption_on {
        return Some(data);
    }
    let mut return_data: Vec<String> = vec![];
    encrypted_id::init("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h61");
    for letter in data.into_bytes() {
        let pub_val = encrypted_id::encrypt(letter as u64, ENCRYPTION.key);
        match pub_val {
            Ok(push) => {
                return_data.push(push);
            }
            Err(e) => {
                error!("failed when encrypting data\n{}\n", e);
                return None;
            }
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
        match id {
            Ok(id) => {
                if let Ok(id) = id.to_string().parse() {
                    return_data.push(id)
                } else {
                    error!("utf8 decryption error occurred");
                    return None;
                }
            }
            Err(e) => {
                error!("failed when decrypting data\n{}\n", e);
                return None;
            }
        }
    }
    match String::from_utf8(return_data){
        Ok(data) => {
            Some(data)
        }
        Err(e) => {
            error!("utf8 decryption error occurred\n{}\n", e);
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
        let client = HostData::get_client().get_connection();
        return match client {
            Ok(mut connection) => {
                Some(connection.set(location, send))
            }
            Err(e) => {
                error!("failed when sending data to redis\n{}\n", e);
                Some(Err(e))
            }
        };
    }
    None
}

pub fn send<T: Display>(val: T) -> Option<RedisResult<bool>> {
    where_send(val, NAME)
}