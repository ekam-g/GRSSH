use crate::client::client_main;
use crate::host::host_main;

mod command;
mod input;
mod db;
mod host;
mod client;

const CLIENT: bool = true;

const REDIS_KEY: &str = "redis://127.0.0.1:6379";

const NAME: &str = "test";

fn main() {
    if CLIENT {
        println!("starting client version");
        client_main()
    } else {
        println!("starting host version");
        host_main();
    }
}