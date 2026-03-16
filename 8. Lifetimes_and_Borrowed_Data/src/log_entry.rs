#[derive(Debug)]
pub enum LogType {
    Disconnected,
    Connected,
    Degraded,
    Unknown,
}

#[derive(Debug)]
pub struct LogEntry<'a> {
    pub service: &'a str,
    pub log_type: LogType,
}

impl LogEntry<'_> {
    pub fn parse(line: &str) -> Result<LogEntry<'_>, String> {
        let logs = line.split_once(":");
        match logs {
            Some((str1, str2)) => {
                let log_entry = LogEntry {
                    service: str1,
                    log_type: get_log_type(str2),
                };
                Ok(log_entry)
            }
            None => Err(format!("{} line skipped!", line)),
        }
    }
    pub fn is_problematic(&self) -> bool {
        match self.log_type {
            LogType::Disconnected | LogType::Degraded | LogType::Unknown => true,
            LogType::Connected => false,
        }
    }

    pub fn is_severe(&self) -> bool {
        match self.log_type {
            LogType::Disconnected => true,
            LogType::Degraded | LogType::Connected | LogType::Unknown => false,
        }
    }

    pub fn print_entry(&self) {
        println!("service={}, status={:?}", self.service, self.log_type);
    }
}

pub fn get_log_type(log_type: &str) -> LogType {
    match log_type {
        "disconnected" => LogType::Disconnected,
        "connected" => LogType::Connected,
        "degraded" => LogType::Degraded,
        _ => LogType::Unknown,
    }
}
