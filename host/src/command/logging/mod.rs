pub struct Log {}

impl Log {
    //noinspection RsConstantConditionIf
    pub fn write(info: String) {
        if crate::LOG {
            let log_try = txt_writer::WriteData {}.add(&info, "log.txt");
            if let Err(e) = log_try {
                println!("failed when logging, trying to recover\n{e}");
                let mut error_amount: i8 = 0;
                loop {
                    let log_try = txt_writer::WriteData {}.replace(&info, "log.txt");
                    if let Err(e) = log_try {
                        println!("failed when logging, trying to recover: {error_amount}\n{e}");
                        error_amount += 1;
                        if error_amount > 120 {
                            return;
                        }
                    } else {
                        println!("logged successfully");
                        return;
                    }
                }
            }
        }
    }
    //noinspection RsConstantConditionIf
    pub fn read(command: &str) -> Option<String> {
        if command.contains("view-logs") {
            return if crate::LOG {
                let reader = txt_writer::ReadData {};
                let error = reader.read_one("log.txt");
                match error {
                    Ok(data) => {
                        Some(data)
                    }
                    Err(e) => {
                        Some(format!("failed to read logs\n{e}"))
                    }
                }
            } else {
                Some("Logs are Disabled".to_owned())
            }
        }
        None
    }
}
