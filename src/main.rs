mod command;
mod input;
mod db;

use crate::command::exc;
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
            println!("{}", exc(get()));
        }
    }
}