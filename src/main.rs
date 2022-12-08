use crate::client::client_main;
use crate::host::host_main;

mod command;
mod input;
mod db;
mod host;
mod client;
pub mod ram_var;


const LOCATION_TO_REDIS_KEY: &str = "redis_key.txt";

const NAME: &str = "test";

fn main() {
    if input::y_n("y for host, n client") {
        println!("starting client version");
        client_main()
    } else {
        println!("starting host version");
        host_main();
    }
}
