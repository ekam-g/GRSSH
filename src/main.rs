mod command;
mod input;
mod db;
mod host;
mod client;

use crate::command::exc;
use crate::host::host_main;
use crate::input::get;

const CLIENT: bool = false;

const REDIS_KEY : &str = "";

const NAME : &str = "test";

fn main() {
    if CLIENT {
        loop {
            todo!()
        }
    } else {
        loop {
            host_main();
        }
    }
}