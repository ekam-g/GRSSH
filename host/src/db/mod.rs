use std::fs;

use redis::{IntoConnectionInfo, RedisResult, ToRedisArgs};
use redis::{Client, Commands};

use crate::ENCRYPTION;
use crate::ram_var::HostData;

pub mod get_command_thread;


//TODO refactor soon

pub fn make_client<T: IntoConnectionInfo>(redis_key: T) -> RedisResult<Client> {
    Client::open(redis_key)
}

pub fn send<T: ToRedisArgs>(val: T) -> RedisResult<bool> {
    let client = HostData::get();
    client
        .client
        .get_connection()?
        .set(crate::NAME, val)
}

pub fn format_path(passed: Vec<String>) -> String {
    if cfg!(windows) {
        return format!("C:{}", passed.join("\\"));
    }
    format!("/{}", passed.join("/"))
}

pub fn send_path(val: String) -> RedisResult<bool> {
    let client = HostData::get();
    client
        .client
        .get_connection()?
        .set(path(), val)
}

pub fn path() -> String {
    format!("{}location", crate::NAME)
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
    let _: RedisResult<bool> = good_client.set(crate::NAME, "**unable to find old path, please cd into home directory");
    let _: RedisResult<bool> = good_client.set(format!("{}location", crate::NAME), "/");
}

pub fn get() -> RedisResult<String> {
    let client = HostData::get();
    client
        .client
        .get_connection()?
        .get(crate::NAME)
}

pub fn encrypt(data: String) -> String{
    let mut return_data:Vec<String> = vec![];
    encrypted_id::init("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h61");
    for letter in data.into_bytes(){
        return_data.push( encrypted_id::encrypt(letter as u64, ENCRYPTION.key).unwrap());
    }
    dbg!(&return_data);
    return_data.join("/")
}

pub fn decrypt(data: String) -> String{
    let mut return_data:Vec<u8> = vec![];
    encrypted_id::init("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h61");
    for letter in data.split('/') {
        let id = encrypted_id::decrypt(letter, ENCRYPTION.key).unwrap();
        return_data.push(id.to_string().parse().unwrap())
    }
    String::from_utf8(return_data).unwrap()
}

pub struct Encrypt<'a> {
    pub key: &'a str,
}