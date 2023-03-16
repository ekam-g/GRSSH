use std::{env, thread};
use std::process::exit;
use std::time::Duration;
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
    loop {
        let send_result = db::send("**Started Server");
        if let Some(Err(e)) = send_result {
            println!("Failed starting server when connecting to redis\n{e}");
            break
        }
        thread::sleep(Duration::from_secs(1));
    }
    info!("Config checks passed\n");
}

fn show_error() {
    println!("console logging is off please use [export RUST_LOG=trace] to enable")
}