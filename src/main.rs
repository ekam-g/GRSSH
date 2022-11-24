mod command;
mod input;
mod db;
mod host;
mod client;

use crate::client::client_main;
use crate::host::host_main;

const CLIENT: bool = true;

const REDIS_KEY: &str = "redis://127.0.0.1:6379";

const NAME : &str = "test";

fn main() {
    if CLIENT {
        loop {
            client_main()
        }
    } else {
        loop {
            host_main();
        }
    }
}