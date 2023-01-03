extern crate core;

use crate::client::client_main;
use crate::db::{Encrypt, get, send, who};
use crate::ram_var::HostData;

mod client;
mod db;
mod input;
mod ram_var;
const LOCATION_TO_REDIS_KEY: &str = "redis_key.txt";
const ENCRYPTION: Encrypt =  Encrypt{
    key : "hello",
};


fn main(){
    println!("starting client version");
    who();
    client_main()
}
