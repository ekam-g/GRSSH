use std::env;
use std::process::exit;
use crate::config::{LOG, NAME};
use crate::db;

pub fn check() {
    if LOG {
        match env::var("RUST_LOG") {
            Ok(data) => {
                if !data.contains("trace") {
                    show_error();
                }
            }
            Err(_) => {
                env::set_var("RUST_LOG", "trace");
            }
        }
        if  env_logger::builder().try_init().is_err() {
            show_error();
        }
    }
    if NAME.contains("location") {
        println!("please make sure server name does not contain the word location, exiting.....");
        exit(0);
    }
    let send_result = db::send("**Started Server");
    if let Some(Err(e)) = send_result {
        println!("Failed starting server when connecting to redis\n{e}");
        exit(0);
    }
    info!("Config checks passed\n");
}

fn show_error() {
    println!("console logging is off please use [export RUST_LOG=trace] to enable")
}