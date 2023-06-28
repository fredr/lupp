pub mod json;
pub mod logfmt;

pub enum LogFormat {
    Json,
    Logfmt,
    Colored,
    Unknown,
}

pub fn detect(line: &str) -> LogFormat {
    // if the line already contains colors, skip it
    if line.contains("\x1b") {
        return LogFormat::Colored;
    }

    // TODO: be smarter?
    if line.starts_with('{') {
        return LogFormat::Json;
    }

    // detect logfmt by trying to get first key
    // TODO: be smarter?
    let mut found_key = false;
    for ch in line.chars() {
        if ch.is_ascii_alphabetic() {
            found_key = true;
        } else {
            if ch == '=' && found_key {
                return LogFormat::Logfmt;
            }
            return LogFormat::Unknown;
        }
    }

    LogFormat::Unknown
}
