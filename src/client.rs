use crate::db::send;
use crate::input::get;

pub fn client_main() {
    loop {
        let user_input = get();
        let error = send(&user_input);
        match error {
            Ok(_) => {}
            Err(_) => {
                println!("error when connecting to redis")
            }
        }
    }
}