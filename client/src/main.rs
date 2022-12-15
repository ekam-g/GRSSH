use crate::client::client_main;
mod client;
mod db;
pub mod ram_var;
mod input;

const LOCATION_TO_REDIS_KEY: &str = "redis_key.txt";

const NAME: &str = "test";

fn main() {
    db::client().expect("Please check your connection or your redis key");
    println!("starting client version");
    client_main()
}
