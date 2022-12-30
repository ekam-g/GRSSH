use crate::client::client_main;
use crate::db::who;

mod client;
mod db;
mod ram_var;
mod input;

const LOCATION_TO_REDIS_KEY: &str = "redis_key.txt";


fn main() {
    println!("starting client version");
    who();
    client_main()
}
