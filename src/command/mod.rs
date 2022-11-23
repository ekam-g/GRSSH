use std::process::Command;

pub fn exc(what: String) -> String {
    let (first, rest) = {
        let (mut first, mut rest) = ("", vec![]);
        let mut first_done = false;
        for word in what.split_whitespace() {
            if !first_done {
                first = word;
                first_done = true;
            } else {
                rest.push(word)
            }
        }
        (first, rest)
    };
    let success = Command::new(first).args(rest).output();
    match success {
        Ok(good) => {
            String::from_utf8(good.stdout).unwrap()
        }
        Err(error) => {
            let val = String::from_utf8(
                Command::new("bash")
                    .args(["-c"])
                    .args(what.split_whitespace())
                    .output()
                    .unwrap_or(
                        Command::new("cmd")
                        .args(["/C"])
                        .args(what.split_whitespace())
                        .output()
                        .expect("error when running code")
                    )
                    .stdout
            ).unwrap();
            if val == "".to_owned() {
                return error.to_string();
            }
            val
        }
    }
}
