use std::io;

thread_local! {
    static THEME: Theme = Theme::default();
}

struct Theme {
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
            dim: StyleBuilder::new().bold().color_256(242).build(),
            trace: StyleBuilder::new().color_magenta().bold().build(),
            debug: StyleBuilder::new().color_blue().bold().build(),
            info: StyleBuilder::new().color_green().bold().build(),
            warn: StyleBuilder::new().color_yellow().bold().build(),
            error: StyleBuilder::new().color_red().bold().build(),
            fatal: StyleBuilder::new().color_red().bold().build(),
            info_text: StyleBuilder::new().color_cyan().build(),
            error_text: StyleBuilder::new().color_red().build(),
            debug_text: StyleBuilder::new().color_magenta().build(),
        }
    }
}

pub fn write_highlighted(text: &str, writer: &mut impl io::Write) -> io::Result<()> {
    THEME.with(|theme| theme.highlight.write(text, writer))
}
pub fn write_dimmed(text: &str, writer: &mut impl io::Write) -> io::Result<()> {
    THEME.with(|theme| theme.dim.write(text, writer))
}

pub fn write_key(key: &str, writer: &mut impl io::Write) -> io::Result<()> {
    THEME.with(|theme| match key {
        "severity" | "level" | "lvl" | "msg" | "message" | "trace_id" | "span_path" => {
            theme.highlight.write(key, writer)
        }
        "error" | "err" => theme.error.write(key, writer),
        _ => theme.dim.write(key, writer),
    })
}

pub fn write_value(key: &str, value: &str, writer: &mut impl io::Write) -> io::Result<()> {
    THEME.with(|theme| match key {
        "level" | "lvl" | "severity" => {
            let style = match value {
                "trace" | "Trace" | "TRACE" => &theme.trace,
                "debug" | "Debug" | "DEBUG" => &theme.debug,
                "info" | "Info" | "INFO" => &theme.info,
                "warn" | "Warn" | "WARN" | "warning" | "Warning" | "WARNING" => &theme.warn,
                "error" | "Error" | "ERROR" => &theme.error,
                "fatal" | "Fatal" | "FATAL" => &theme.fatal,
                _ => &theme.dim,
            };

            style.write(value, writer)
        }
        "msg" | "message" => theme.info_text.write(value, writer),
        "error" | "err" => theme.error_text.write(value, writer),
        "trace_id" | "span_path" => theme.debug_text.write(value, writer),
        _ => theme.dim.write(value, writer),
    })
}

// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797#colors--graphics-mode

pub enum AnsiColor {
    RGB(u8, u8, u8),
    Color16(u8),
    Color256(u8),
}

#[derive(Default)]
pub struct Style {
    bold: bool,
    color: Option<AnsiColor>,
}

impl Style {
    pub fn write(&self, text: &str, writer: &mut impl io::Write) -> io::Result<()> {
        if self.bold {
            writer.write_all(b"\x1b[1m")?;
        }

        if let Some(color) = &self.color {
            match color {
                AnsiColor::RGB(r, g, b) => {
                    // \x1b[38;2;{r};{g};{b}m
                    writer.write_all(
                        &[
                            b"\x1b[38;2;",
                            r.to_string().as_bytes(),
                            b";",
                            g.to_string().as_bytes(),
                            b";",
                            b.to_string().as_bytes(),
                            b"m",
                        ]
                        .concat(),
                    )?
                }
                AnsiColor::Color16(code) => {
                    writer.write_all(&[b"\x1b[", code.to_string().as_bytes(), b"m"].concat())?
                }
                AnsiColor::Color256(code) => writer
                    .write_all(&[b"\x1b[38;5;", code.to_string().as_bytes(), b"m"].concat())?,
            }
        }

        writer.write_all(text.as_bytes())?;
        writer.write_all(b"\x1b[0m")?;

        Ok(())
    }
}

#[derive(Default)]
pub struct StyleBuilder {
    style: Style,
}

impl StyleBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn build(self) -> Style {
        self.style
    }

    pub fn bold(mut self) -> Self {
        self.style.bold = true;
        self
    }

    pub fn color_black(mut self) -> Self {
        self.style.color = Some(AnsiColor::Color16(30));
        self
    }
    pub fn color_red(mut self) -> Self {
        self.style.color = Some(AnsiColor::Color16(31));
        self
    }
    pub fn color_green(mut self) -> Self {
        self.style.color = Some(AnsiColor::Color16(32));
        self
    }
    pub fn color_yellow(mut self) -> Self {
        self.style.color = Some(AnsiColor::Color16(33));
        self
    }
    pub fn color_blue(mut self) -> Self {
        self.style.color = Some(AnsiColor::Color16(34));
        self
    }
    pub fn color_magenta(mut self) -> Self {
        self.style.color = Some(AnsiColor::Color16(35));
        self
    }
    pub fn color_cyan(mut self) -> Self {
        self.style.color = Some(AnsiColor::Color16(36));
        self
    }
    pub fn color_white(mut self) -> Self {
        self.style.color = Some(AnsiColor::Color16(37));
        self
    }

    pub fn color_256(mut self, color: u8) -> Self {
        self.style.color = Some(AnsiColor::Color256(color));
        self
    }
}
