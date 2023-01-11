extern crate core;
#[macro_use]
extern crate log;

use crate::db::checks::check;
use crate::db::sentry_logging::enable_sentry;
use crate::host::host_main;

mod command;
mod db;
mod host;
pub mod ram_var;
mod config;

fn main() {
    check();
    let _panic_watcher = enable_sentry();
    host_main();
}
