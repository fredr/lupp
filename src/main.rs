use lupp::format::{self, json, logfmt, LogFormat};

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lines() {
        let line = line.unwrap();

        let mut line = match format::detect(&line) {
            LogFormat::Json => json::enhance(&line),
            LogFormat::Logfmt => logfmt::enhance(&line),
            LogFormat::Unknown | LogFormat::Colored => line,
        };

        line.push('\n');
        stdout.write(line.as_bytes())?;
    }

    Ok(())
}
