
use redis::{IntoConnectionInfo, RedisResult};
use redis::{Client, Commands, Connection};

pub fn client() -> RedisResult<Connection> {
    try_client(crate::ram_var::HostData::get().redis_key.clone())
}
pub fn try_client<T : IntoConnectionInfo>(redis_key : T) -> RedisResult<Connection> {
    let redis = Client::open(redis_key)?;
    redis.get_connection()
}

pub fn send(val: &String) -> RedisResult<bool> {
    let mut client = client()?;
    client.set_ex(crate::NAME, val, 360)
}

pub fn get() -> RedisResult<String> {
    let mut client = client()?;
    client.get(crate::NAME)
}
pub fn get_path() -> RedisResult<String> {
    let mut  client = client()?;
    client.get(path())
}
pub fn path() -> String {
    format!("{}location", crate::NAME)
}