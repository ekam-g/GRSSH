extern crate core;

use crate::host::host_main;

mod command;
mod db;
mod host;
pub mod ram_var;

const SHELL: &str = "zsh";
const LOCATION_TO_REDIS_KEY: &str = "redis_key.txt";

const NAME: &str = "test";

fn main() {
    db::send("**Started Server").expect("Error When Connecting To Redis");
    println!("starting host version");
    host_main();
}
