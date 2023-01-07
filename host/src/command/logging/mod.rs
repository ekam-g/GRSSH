pub struct Log {}

impl Log {
    //noinspection RsConstantConditionIf
    pub fn write(info: String) {
        if crate::config::LOG {
            let log_try = txt_writer::WriteData {}.add(&info, "log.txt");
            if let Err(e) = log_try {
                error!("failed when logging, trying to recover\n{e}\n");
                let mut error_amount: i8 = 0;
                loop {
                    let log_try = txt_writer::WriteData {}.replace(&info, "log.txt");
                    if let Err(e) = log_try {
                        warn!("failed when logging, trying to recover: {error_amount}\n{e}\n");
                        error_amount += 1;
                        if error_amount > 120 {
                            error!("failed to log\n");
                            return;
                        }
                    } else {
                        info!("logged successfully\n");
                        return;
                    }
                }
            }
        }
    }
    //noinspection RsConstantConditionIf
    pub fn read(command: &str) -> Option<String> {
        if command.contains("view-logs") {
            warn!("Logs where Requested\n");
            return if crate::config::LOG {
                let reader = txt_writer::ReadData {};
                let error = reader.read_one("log.txt");
                match error {
                    Ok(data) => {
                        warn!("Logs Sent\n");
                        Some(data)
                    }
                    Err(e) => {
                        error!("Log Where Unable to be Sent\n{}\n", e);
                        Some(format!("failed to read logs\n{e}"))
                    }
                }
            } else {
                error!("Logs are Disabled\n");
                Some("Logs are Disabled".to_owned())
            }
        }
        None
    }
}
