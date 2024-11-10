enum LogType {
    Info,
    Warn,
    Error,
}

pub struct Log {}

impl Log {
    pub fn info(log: String) {
        raw_log(LogType::Info, None, log)
    }

    pub fn warn(log: String) {
        raw_log(LogType::Warn, None, log)
    }

    pub fn error(log: String) {
        raw_log(LogType::Error, None, log)
    }
}

pub struct CustomLog {
    prefix: String,
}

impl CustomLog {
    pub fn new(prefix: String) -> Self {
        return CustomLog { prefix: prefix };
    }

    pub fn info(&self, log: String) {
        raw_log(LogType::Info, Some(self.prefix.clone()), log)
    }

    pub fn warn(&self, log: String) {
        raw_log(LogType::Warn, Some(self.prefix.clone()), log)
    }

    pub fn error(&self, log: String) {
        raw_log(LogType::Error, Some(self.prefix.clone()), log)
    }
}

fn raw_log(log_type: LogType, prefix: Option<String>, log: String) {
    let mut _prefix = String::from("");
    if let Some(prefix) = prefix {
        _prefix = prefix
    }

    println!("[{}] {} {}", get_log_type_text(log_type), _prefix, log)
}

fn get_log_type_text(log_type: LogType) -> String {
    match log_type {
        LogType::Info => "Info".to_string(),
        LogType::Warn => "Warn".to_string(),
        LogType::Error => "Error".to_string(),
    }
}
