const WHITE: &str = "\x1b[0;97m";
const GREEN: &str = "\x1b[32m";
const GRAY: &str = "\x1b[38;5;243m";
const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[1;34m";
const YELLOW: &str = "\x1b[33m";
const PURPLE: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";

const RESET: &str = "\x1b[0m";

pub enum Color {
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

pub fn write_with_color(color: Color, s: &str, line: &mut String) {
    line.push_str(color.code());
    line.push_str(s);
    line.push_str(RESET);
}

pub fn write_key(key: &str, line: &mut String) {
    match key {
        "severity" | "level" | "lvl" | "msg" | "message" | "time" | "ts" | "timestamp"
        | "trace_id" | "span_path" => write_with_color(Color::White, key, line),
        "error" | "err" => write_with_color(Color::Red, key, line),
        _ => write_with_color(Color::Gray, key, line),
    }
}

pub fn write_value(key: &str, value: &str, line: &mut String) {
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

            write_with_color(c, value, line);
        }
        "msg" | "message" => write_with_color(Color::Cyan, value, line),
        "error" | "err" => write_with_color(Color::Red, value, line),
        "time" | "ts" | "timestamp" => write_with_color(Color::White, value, line),
        "trace_id" | "span_path" => write_with_color(Color::Purple, value, line),
        _ => write_with_color(Color::Gray, value, line),
    }
}
