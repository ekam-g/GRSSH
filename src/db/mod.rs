pub mod get_command_thread;

use redis::{Client, Commands, Connection};
use redis::RedisResult;

fn client() -> RedisResult<Connection> {
    let redis = Client::open(
        crate::
        ram_var::
        HostData::
        get()
        .redis_ket
        .clone()
    )?;
    redis.get_connection()
}

pub fn send(val: &String) -> RedisResult<bool> {
    let mut client = client()?;
    client.set(&crate::NAME, val)
}

pub fn get() -> RedisResult<String> {
    let mut client = client()?;
    client.get(&crate::NAME)
}
