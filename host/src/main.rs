extern crate core;

use std::process::exit;
use config::NAME;
use crate::db::sentry_logging::enable_sentry;
use crate::host::host_main;

mod command;
mod db;
mod host;
pub mod ram_var;
mod config;

fn main() {
    let _panic_watcher = enable_sentry();
    {
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
    host_main();
}
