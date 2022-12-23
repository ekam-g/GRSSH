use std::fs;
use std::process::Command;
use crate::db::format_path;


use crate::ram_var::HostData;

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
    let file: String = {
        let mut path = HostData::get();
        if fs::read_dir(format_path(path.location.clone())).is_err() {
            path.location = path.last_working_location.clone();
        }else {
            path.last_working_location = path.location.clone();
        }

        format_path(path.location.clone())
    };
    let success = Command::new(first).current_dir(&file)
        .args(rest)
        .output();
    match success {
        Ok(good) => {
            String::from_utf8(good.stdout).unwrap()
        }
        Err(error) => {
            let run = {
                if crate::SHELL.is_empty() {
                    if !cfg!(target_os = "linux") {
                        Command::new("bash")
                            .current_dir(file)
                            .args(["-c"])
                            .args(what.split_whitespace())
                            .output()
                    } else {
                        Command::new("cmd")
                            .current_dir(file)
                            .args(["/C"])
                            .args(what.split_whitespace())
                            .output()
                    }
                } else {
                    Command::new(crate::SHELL)
                        .current_dir(file)
                        .args(["/C"])
                        .args(what.split_whitespace())
                        .output()
                }
            };
            match run {
                Ok(data) => {
                    let good_or_no = String::from_utf8(data.stdout).unwrap();
                    if good_or_no == *"" {
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

