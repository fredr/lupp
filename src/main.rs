use lupp::{
    format::{self, json, logfmt, LogFormat},
    styling,
};

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let theme = styling::Theme::default();

    for line in stdin.lines() {
        let line = line.unwrap();

        match format::detect(&line) {
            LogFormat::Json => json::enhance(&theme, &line, &mut stdout)?,
            LogFormat::Logfmt => logfmt::enhance(&theme, &line, &mut stdout)?,
            LogFormat::Unknown | LogFormat::Colored => stdout.write_all(line.as_bytes())?,
        };

        // write a newline as the lines iterator strips that away
        stdout.write_all(&[b'\n'])?;
    }

    Ok(())
}
