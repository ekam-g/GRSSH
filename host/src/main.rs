extern crate core;

use std::process::exit;
use config::NAME;
use crate::db::checks::check;
use crate::db::sentry_logging::enable_sentry;
use crate::host::host_main;

mod command;
mod db;
mod host;
pub mod ram_var;
mod config;

fn main() {
    let _panic_watcher = enable_sentry();
    check();
    host_main();
}
