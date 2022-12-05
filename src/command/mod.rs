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
            let run = {
                if !cfg!(target_os = "linux") {
                    Command::new("bash")
                        .args(["-c"])
                        .args(what.split_whitespace())
                        .output()
                } else {
                    Command::new("cmd")
                        .args(["/C"])
                        .args(what.split_whitespace())
                        .output()
                }
            };
            match run {
                Ok(data) => {
                    let good_or_no = String::from_utf8(data.stdout).unwrap();
                    if good_or_no == "".to_owned() {
                        return error.to_string();
                    }
                    good_or_no
                }
                Err(e) => {
                    e.to_string()
                }
            }
        }
    }
}