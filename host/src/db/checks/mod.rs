use std::process::exit;
use crate::config::NAME;
use crate::db;

pub fn check() {
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
}