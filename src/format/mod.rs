pub mod json;
pub mod logfmt;

#[derive(PartialEq, Debug)]
pub enum LogFormat {
    Json,
    Logfmt,
    Colored,
    Unknown,
}

pub fn detect(line: &str) -> LogFormat {
    // if the line already contains colors, skip it
    if line.contains('\x1b') {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format() {
        let tests = [
            (
                r#"field=thing other="hello world" status=200"#,
                LogFormat::Logfmt,
            ),
            (
                r#"{"field": "value", "other": "hello world", "status": 200}"#,
                LogFormat::Json,
            ),
            (
                r#"{"field": "value", "[37mother": "hello world", "status": 200}[0m"#,
                LogFormat::Colored,
            ),
            (
                r#"This is not a strucutured log line, just some text"#,
                LogFormat::Unknown,
            ),
        ];

        for (log_row, expected_format) in tests {
            assert_eq!(detect(log_row), expected_format)
        }
    }
}
