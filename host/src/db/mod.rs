use std::fmt::Display;
use std::fs;

use redis::{IntoConnectionInfo, RedisResult, ToRedisArgs};
use redis::{Client, Commands};

use crate::config::{ENCRYPTION, NAME};
use crate::ram_var::HostData;

pub mod get_command_thread;


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

pub fn get_path(redis_location: String) -> Vec<String> {
    let mut return_val: Vec<String> = vec![];
    let mut err: i8 = 0;
    loop {
        let client = make_client(redis_location.clone());
        if let Ok(mut good_client) = client {
            match good_client.get(path()) {
                Ok(data) => {
                    let check  = decrypt(data);
                    if let Some(check) = check {
                        if fs::read_dir(check.clone()).is_ok() {
                            for path in check.split('/') {
                                return_val.push(path.to_owned());
                            }
                            return return_val;
                        }
                    }
                    return set_unknown(good_client);
                }
                Err(_) => {
                    err += 1;
                    if err == 30 {
                        return set_unknown(good_client);
                    }
                }
            }
        }
    }
}

fn set_unknown(mut good_client: Client) -> Vec<String> {
    println!("unable to find old path, please cd into home directory");
    let _: RedisResult<bool> = good_client.set(NAME, encrypt("**unable to find old path, please cd into home directory".to_owned()));
    let _: RedisResult<bool> = good_client.set(format!("{NAME}location"), encrypt("/".to_owned()));
    vec![]
}

pub fn get() -> RedisResult<Option<String>> {
    let client = HostData::get_client().get_connection()?.get(NAME);
    match client {
        Err(e)=> Err(e),
        Ok(data)=> {
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
    Some(String::from_utf8(return_data).unwrap())
}

pub struct Encrypt<'a> {
    pub key: &'a str,
    pub encryption_on: bool,
}


fn where_send<T : Display, E : ToRedisArgs>(val: T, location : E) -> Option<RedisResult<bool>>{
    let send = encrypt(val.to_string());
    if let Some(send) = send {
        let client = HostData::get_client().get_connection();
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
    where_send(val, NAME)
}