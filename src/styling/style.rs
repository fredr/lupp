use std::io;

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
                AnsiColor::Rgb(r, g, b) => {
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
                    )?;
                }
                AnsiColor::Color16(code) => {
                    writer.write_all(&[b"\x1b[", code.to_string().as_bytes(), b"m"].concat())?;
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

    pub fn color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style.color = Some(AnsiColor::Rgb(r, g, b));
        self
    }
}

// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797#colors--graphics-mode

pub enum AnsiColor {
    Rgb(u8, u8, u8),
    Color16(u8),
    Color256(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_styles() {
        let tests = [
            (
                StyleBuilder::new().bold().build(),
                "text",
                "\x1b[1mtext\x1b[0m",
            ),
            (
                StyleBuilder::new().color_red().build(),
                "text",
                "\x1b[31mtext\x1b[0m",
            ),
            (
                StyleBuilder::new().color_256(214).build(),
                "text",
                "\x1b[38;5;214mtext\x1b[0m",
            ),
            (
                StyleBuilder::new().color_rgb(200, 100, 0).bold().build(),
                "text",
                "\x1b[1m\x1b[38;2;200;100;0mtext\x1b[0m",
            ),
        ];

        for (style, input, output) in tests {
            let mut writer = Vec::new();
            style
                .write(input, &mut writer)
                .expect("couldn't write to writer");
            assert_eq!(
                String::from_utf8(writer).expect("couldn't create string from writer"),
                output
            );
        }
    }
}
