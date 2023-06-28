use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lines() {
        let line = line.unwrap();

        let mut line = match detect_type(&line) {
            LogFormat::Json => handle_json(&line),
            LogFormat::Logfmt => handle_logfmt(&line),
            LogFormat::Unknown | LogFormat::Colored => line,
        };

        line.push('\n');
        stdout.write(line.as_bytes())?;
    }

    Ok(())
}

enum LogFormat {
    Json,
    Logfmt,
    Colored,
    Unknown,
}

fn detect_type(line: &str) -> LogFormat {
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

enum JsonContext {
    None,
    Value,
    ValueString,
    ValueNumber,
    Key,
}
struct JsonState {
    line: String,
    current: String,
    current_key: String,
    context: JsonContext,
    escaped: bool,
}

impl JsonState {
    fn new() -> Self {
        Self {
            line: String::new(),
            current: String::new(),
            current_key: String::new(),
            context: JsonContext::None,
            escaped: false,
        }
    }
}

const WHITE: &str = "\x1b[0;97m";
const GREEN: &str = "\x1b[32m";
const GRAY: &str = "\x1b[38;5;243m";
const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[1;34m";
const YELLOW: &str = "\x1b[33m";
const PURPLE: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";

const RESET: &str = "\x1b[0m";

enum Color {
    White,
    Green,
    Gray,
    Red,
    Blue,
    Cyan,
    Purple,
    Yellow,
}

// TODO: bold, underline etc, how to handle nicely?
impl Color {
    fn code(&self) -> &'static str {
        match self {
            Color::White => WHITE,
            Color::Green => GREEN,
            Color::Gray => GRAY,
            Color::Red => RED,
            Color::Blue => BLUE,
            Color::Yellow => YELLOW,
            Color::Cyan => CYAN,
            Color::Purple => PURPLE,
        }
    }
}

fn color(color: Color, s: &str, line: &mut String) {
    line.push_str(color.code());
    line.push_str(s);
    line.push_str(RESET);
}

enum LogfmtContext {
    Key,
    Value,
}

struct LogfmtState {
    line: String,
    current: String,
    current_key: String,
    context: LogfmtContext,
    escaped: bool,
    quoted: bool,
}

impl LogfmtState {
    fn new() -> Self {
        Self {
            line: String::new(),
            current: String::new(),
            current_key: String::new(),
            context: LogfmtContext::Key,
            escaped: false,
            quoted: false,
        }
    }
}

fn write_key(key: &str, line: &mut String) {
    match key {
        "severity" | "level" | "msg" | "message" | "time" | "ts" | "timestamp" | "trace_id"
        | "span_path" => color(Color::White, key, line),
        "error" | "err" => color(Color::Red, key, line),
        _ => color(Color::Gray, key, line),
    }
}
fn write_value(key: &str, value: &str, line: &mut String) {
    match key {
        "level" | "lvl" | "severity" => {
            let c = match value {
                "trace" | "Trace" | "TRACE" => Color::Purple,
                "debug" | "Debug" | "DEBUG" => Color::Blue,
                "info" | "Info" | "INFO" => Color::Green,
                "warn" | "Warn" | "WARN" | "warning" | "Warning" | "WARNING" => Color::Yellow,
                "error" | "Error" | "ERROR" => Color::Red,
                "fatal" | "Fatal" | "FATAL" => Color::Red,
                _ => Color::Gray,
            };

            color(c, value, line);
        }
        "msg" | "message" => color(Color::Cyan, value, line),
        "error" | "err" => color(Color::Red, value, line),
        "time" | "ts" | "timestamp" => color(Color::White, value, line),
        "trace_id" | "span_path" => color(Color::Purple, value, line),
        _ => color(Color::Gray, value, line),
    }
}

fn handle_logfmt(line: &str) -> String {
    let mut state = line.chars().fold(LogfmtState::new(), |mut state, ch| {
        match state.context {
            LogfmtContext::Value => {
                if state.escaped {
                    state.escaped = false;
                    state.current.push(ch);
                } else {
                    match ch {
                        '\\' => {
                            state.escaped = true;
                            state.current.push(ch);
                        }
                        '"' => {
                            state.quoted = !state.quoted;
                            state.current.push(ch);
                        }
                        ' ' if !state.quoted => {
                            write_value(&state.current_key, &state.current, &mut state.line);
                            state.line.push(' ');

                            // reset state
                            state.current = String::new();
                            state.current_key = String::new();
                            state.context = LogfmtContext::Key;
                        }
                        _ => state.current.push(ch),
                    }
                }
            }
            LogfmtContext::Key => match ch {
                '=' => {
                    write_key(&state.current, &mut state.line);
                    state.line.push('=');

                    state.current_key = state.current;

                    // reset state
                    state.current = String::new();
                    state.context = LogfmtContext::Value;
                }
                _ => state.current.push(ch),
            },
        }

        state
    });

    // write the last value
    write_value(&state.current_key, &state.current, &mut state.line);

    state.line
}

fn handle_json(line: &str) -> String {
    let state = line.chars().fold(JsonState::new(), |mut state, ch| {
        match state.context {
            JsonContext::None => match ch {
                '{' | '}' | ',' => color(Color::White, &ch.to_string(), &mut state.line),
                '"' => state.context = JsonContext::Key,
                ':' => {
                    state.line.push(':');
                    state.context = JsonContext::Value
                }
                ch => state.line.push(ch),
            },
            JsonContext::Value => match ch {
                // TODO: value object
                // TODO: handle array
                // TODO: boolean
                '"' => state.context = JsonContext::ValueString,
                '0'..='9' => state.context = JsonContext::ValueNumber,
                ch => state.line.push(ch),
            },
            JsonContext::Key => {
                if state.escaped {
                    state.escaped = false;
                    state.current.push(ch);
                } else {
                    match ch {
                        '\\' => {
                            state.escaped = true;
                            state.current.push(ch);
                        }
                        '"' => {
                            color(Color::Gray, "\"", &mut state.line);
                            write_key(&state.current, &mut state.line);
                            color(Color::Gray, "\"", &mut state.line);

                            state.current_key = state.current;

                            // reset state
                            state.current = String::new();
                            state.context = JsonContext::None;
                        }
                        ch => state.current.push(ch),
                    }
                }
            }
            JsonContext::ValueString => {
                if state.escaped {
                    state.escaped = false;
                    state.current.push(ch);
                } else {
                    match ch {
                        '\\' => {
                            state.escaped = true;
                            state.current.push(ch);
                        }
                        '"' => {
                            color(Color::Gray, "\"", &mut state.line);
                            write_value(&state.current_key, &state.current, &mut state.line);
                            color(Color::Gray, "\"", &mut state.line);

                            // reset state
                            state.current_key = String::new();
                            state.current = String::new();
                            state.context = JsonContext::None;
                        }
                        ch => state.current.push(ch),
                    }
                }
            }
            JsonContext::ValueNumber => match ch {
                ',' | '}' => {
                    color(Color::Gray, &state.current, &mut state.line);
                    color(Color::White, &ch.to_string(), &mut state.line);

                    // reset state
                    state.current_key = String::new();
                    state.current = String::new();
                    state.context = JsonContext::None;
                }
                ch => state.current.push(ch),
            },
        }
        state
    });

    state.line
}
