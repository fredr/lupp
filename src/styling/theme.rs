use std::io;

use super::style::{Style, StyleBuilder};

pub struct Theme {
    highlight: Style,
    dim: Style,

    trace: Style,
    debug: Style,
    info: Style,
    warn: Style,
    error: Style,
    fatal: Style,

    info_text: Style,
    error_text: Style,
    debug_text: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            highlight: StyleBuilder::new().color_white().build(),
            dim: StyleBuilder::new().color_256(242).build(),
            trace: StyleBuilder::new().color_magenta().bold().build(),
            debug: StyleBuilder::new().color_blue().bold().build(),
            info: StyleBuilder::new().color_green().bold().build(),
            warn: StyleBuilder::new().color_yellow().bold().build(),
            error: StyleBuilder::new().color_red().bold().build(),
            fatal: StyleBuilder::new().color_rgb(255, 0, 0).bold().build(),
            info_text: StyleBuilder::new().color_256(45).build(),
            error_text: StyleBuilder::new().color_red().build(),
            debug_text: StyleBuilder::new().color_magenta().build(),
        }
    }
}

impl Theme {
    pub fn write_highlighted(&self, text: &str, writer: &mut impl io::Write) -> io::Result<()> {
        self.highlight.write(text, writer)
    }
    pub fn write_dimmed(&self, text: &str, writer: &mut impl io::Write) -> io::Result<()> {
        self.dim.write(text, writer)
    }

    pub fn write_key(&self, key: &str, writer: &mut impl io::Write) -> io::Result<()> {
        match key {
            "severity" | "level" | "lvl" | "msg" | "message" | "status" | "status_code"
            | "trace_id" | "span_path" | "span" => self.highlight.write(key, writer),
            "error" | "err" => self.error.write(key, writer),
            _ => self.dim.write(key, writer),
        }
    }

    pub fn write_value(
        &self,
        key: &str,
        value: &str,
        writer: &mut impl io::Write,
    ) -> io::Result<()> {
        match key {
            "level" | "lvl" | "severity" => {
                let style = match value {
                    "trace" | "Trace" | "TRACE" => &self.trace,
                    "debug" | "Debug" | "DEBUG" => &self.debug,
                    "info" | "Info" | "INFO" => &self.info,
                    "warn" | "Warn" | "WARN" | "warning" | "Warning" | "WARNING" => &self.warn,
                    "error" | "Error" | "ERROR" => &self.error,
                    "fatal" | "Fatal" | "FATAL" => &self.fatal,
                    _ => &self.dim,
                };

                style.write(value, writer)
            }
            "msg" | "message" => self.info_text.write(value, writer),
            "status" | "status_code" => self.highlight.write(value, writer),
            "error" | "err" => self.error_text.write(value, writer),
            "trace_id" | "span_path" | "span" => self.debug_text.write(value, writer),
            _ => self.dim.write(value, writer),
        }
    }
}
