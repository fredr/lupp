use std::io;

use super::style::{AnsiStyle, Style, StyleBuilder};

pub struct Theme<S>
where
    S: Style,
{
    highlight: S,
    dim: S,

    trace: S,
    debug: S,
    info: S,
    warn: S,
    error: S,
    fatal: S,

    info_text: S,
    error_text: S,
    debug_text: S,
}

impl Default for Theme<AnsiStyle> {
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

impl<S: Style> Theme<S> {
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) struct MockStyle(&'static str);
    impl Style for MockStyle {
        fn write(&self, text: &str, writer: &mut impl io::Write) -> io::Result<()> {
            writer.write_all(self.0.as_bytes())?;
            writer.write_all(text.as_bytes())?;

            Ok(())
        }
    }

    pub(crate) fn mock_theme() -> Theme<MockStyle> {
        Theme {
            highlight: MockStyle("[HIGHLIGHT]"),
            dim: MockStyle("[DIM]"),
            trace: MockStyle("[TRACE]"),
            debug: MockStyle("[DEBUG]"),
            info: MockStyle("[INFO]"),
            warn: MockStyle("[WARN]"),
            error: MockStyle("[ERROR]"),
            fatal: MockStyle("[FATAL]"),
            info_text: MockStyle("[INFO_TEXT]"),
            error_text: MockStyle("[ERROR_TEXT]"),
            debug_text: MockStyle("[DEBUG_TEXT]"),
        }
    }
}
