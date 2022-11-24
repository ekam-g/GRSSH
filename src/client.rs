use crate::db::send;
use crate::input::get;

pub fn client_main() {
    loop {
        println!("{}", wait_for_new());
        let user_input = get();
        let error = send(&format!("&&{}", user_input));
        match error {
            Ok(_) => {}
            Err(_) => {
                println!("error when connecting to redis")
            }
        }
    }
}

fn wait_for_new() -> String {
    loop {
        let data = crate::db::get().unwrap();
        if data.contains("**") {
            return data.replace("**", "");
        }
    }
}