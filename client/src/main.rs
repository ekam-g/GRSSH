#![feature(test)]
extern crate core;
extern crate test;

use crate::client::client_main;
use crate::db::{Encrypt, get, send, who};

mod config;
mod client;
mod db;
mod input;
mod ram_var;
mod tests;


fn main(){
    println!("starting client version");
    who();
    client_main()
}
