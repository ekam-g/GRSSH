use std::fmt::Display;
use std::fs;

use redis::{IntoConnectionInfo, RedisResult, ToRedisArgs};
use redis::{Client, Commands};

use crate::{ENCRYPTION, NAME};
use crate::ram_var::HostData;

pub mod get_command_thread;


//TODO refactor soon

pub fn make_client<T: IntoConnectionInfo>(redis_key: T) -> RedisResult<Client> {
    Client::open(redis_key)
}

fn where_send<T : Display, E : ToRedisArgs>(val: T, location : E) -> Option<RedisResult<bool>>{
    let send = encrypt(val.to_string());
    if let Some(send) = send {
        let client = HostData::get().client.get_connection();
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
    format!("{}location", NAME)
}

pub fn get_path(redis_location: String) -> Vec<String> {
    let mut return_val: Vec<String> = vec![];
    let mut err: i8 = 0;
    loop {
        let client = make_client(redis_location.clone());
        if let Ok(mut good_client) = client {
            match good_client.get(path()) {
                Ok(data) => {
                    let check: String = data;
                    if fs::read_dir(check.clone()).is_ok() {
                        for path in check.split('/') {
                            return_val.push(path.to_owned());
                        }
                        return return_val;
                    }
                    println!("unable to find old path, please cd into home directory");
                    set_unknown(good_client);
                    return vec![];
                }
                Err(_) => {
                    err += 1;
                    if err == 30 {
                        println!("unable to find old path, please cd into home directory");
                        set_unknown(good_client);
                        return vec![];
                    }
                }
            }
        }
    }
}

fn set_unknown(mut good_client: Client) {
    let _: RedisResult<bool> = good_client.set(NAME, encrypt("**unable to find old path, please cd into home directory".to_owned()));
    let _: RedisResult<bool> = good_client.set(format!("{}location", NAME), encrypt("/".to_owned()));
}

pub fn get() -> RedisResult<Option<String>> {
    let client = HostData::get();
    let data = client
        .client
        .get_connection()?
        .get(NAME);
    match data {
        Err(e)=> Err(e),
        Ok(data)=> {
           Ok(decrypt(data))
        }
    }

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