use std::io;

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

pub fn write_with_color(color: Color, s: &str, mut writer: impl io::Write) -> io::Result<()> {
    writer.write_all(&[color.code().as_bytes(), s.as_bytes(), RESET.as_bytes()].concat())
}

pub fn write_key(key: &str, writer: impl io::Write) -> io::Result<()> {
    match key {
        "severity" | "level" | "lvl" | "msg" | "message" | "time" | "ts" | "timestamp"
        | "trace_id" | "span_path" => write_with_color(Color::White, key, writer),
        "error" | "err" => write_with_color(Color::Red, key, writer),
        _ => write_with_color(Color::Gray, key, writer),
    }
}

pub fn write_value(key: &str, value: &str, writer: impl io::Write) -> io::Result<()> {
    match key {
        "level" | "lvl" | "severity" => {
            let color = match value {
                "trace" | "Trace" | "TRACE" => Color::Purple,
                "debug" | "Debug" | "DEBUG" => Color::Blue,
                "info" | "Info" | "INFO" => Color::Green,
                "warn" | "Warn" | "WARN" | "warning" | "Warning" | "WARNING" => Color::Yellow,
                "error" | "Error" | "ERROR" => Color::Red,
                "fatal" | "Fatal" | "FATAL" => Color::Red,
                _ => Color::Gray,
            };

            write_with_color(color, value, writer)
        }
        "msg" | "message" => write_with_color(Color::Cyan, value, writer),
        "error" | "err" => write_with_color(Color::Red, value, writer),
        "time" | "ts" | "timestamp" => write_with_color(Color::White, value, writer),
        "trace_id" | "span_path" => write_with_color(Color::Purple, value, writer),
        _ => write_with_color(Color::Gray, value, writer),
    }
}
