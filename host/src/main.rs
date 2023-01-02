extern crate core;

use std::process::exit;
use crate::db::{Encrypt};
use crate::host::host_main;

mod command;
mod db;
mod host;
pub mod ram_var;

const SHELL: &str = "zsh";
const LOCATION_TO_REDIS_KEY: &str = "redis_key.txt";
const LOG: bool = true;
const NAME: &str = "fedora";
const ENCRYPTION: Encrypt =  Encrypt{
    key : "sdaf",
};

fn main() {
    if NAME.contains("location") {
        println!("please make sure server name does not contain location");
        exit(0);
    }
    let send_result = db::send("**Started Server");
    if let Some(Err(e)) = send_result {
        println!("Failed starting server when connecting to redis\n{e}"); 
        exit(0);
    }
    println!("starting host version");
    host_main();
}
