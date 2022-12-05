use std::process::{Command, Child};

pub fn exc(what: String) -> Result<Child,String> {
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
    let success = Command::new(first).args(rest).spawn();
    match success {
        Ok(good) => {
            Ok(good)
        }
        Err(error) => {
            let run = {
                if !cfg!(target_os = "linux") {
                    Command::new("bash")
                        .args(["-c"])
                        .args(what.split_whitespace())
                        .spawn()
                } else {
                    Command::new("cmd")
                        .args(["/C"])
                        .args(what.split_whitespace())
                        .spawn()
                }
            };
            match run {
                Ok(data) => {
                    Ok(data)
                }
                Err(e) => {
                    Err(e.to_string())
                }
            }
        }
    }
}
