pub mod get_command_thread;

use std::fs;
use redis::{IntoConnectionInfo, RedisResult};
use redis::{Client, Commands};
use crate::ram_var::HostData;

//TODO refactor soon

pub fn make_client<T: IntoConnectionInfo>(redis_key: T) ->  RedisResult<Client> {
    Client::open(redis_key)
}

pub fn send(val: &String) -> RedisResult<bool> {
    let client = HostData::get();
    client
        .client
        .get_connection()?
        .set(crate::NAME, val)
}
pub fn format_path(passed :Vec<String>) -> String {
    format!( "/{}", passed.join("/"))
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
                        for path  in check.split('/') {
                            return_val.push(path.to_owned());
                        }
                        return return_val;
                    }
                    println!("unable to find old path, please cd into home directory");
                    set_unknown(good_client);
                    return vec![];
                },
                Err(_) => {
                    err += 1;
                    if err == 30 {
                        println!("unable to find old path, please cd into home directory");
                        set_unknown(good_client);
                        return  vec![]
                    }
                }
            }
        }
    }
}
fn set_unknown(mut good_client : Client) {
    let _ : RedisResult<bool> = good_client.set(crate::NAME,"**unable to find old path, please cd into home directory");
    let _ : RedisResult<bool> = good_client.set(format!("{}location", crate::NAME),"/");
}

pub fn get() -> RedisResult<String> {
    let client = HostData::get();
    client
        .client
        .get_connection()?
        .get(crate::NAME)
}
