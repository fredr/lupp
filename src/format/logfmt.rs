use std::io;

use crate::styling::Theme;

enum Context {
    Key,
    Value,
}

struct State {
    current: String,
    current_key: String,
    context: Context,
    escaped: bool,
    quoted: bool,
}

impl State {
    fn new() -> Self {
        Self {
            current: String::new(),
            current_key: String::new(),
            context: Context::Key,
            escaped: false,
            quoted: false,
        }
    }
}

pub fn enhance(theme: &Theme, line: &str, writer: &mut impl io::Write) -> io::Result<()> {
    let state = line.chars().try_fold(State::new(), |mut state, ch| {
        match state.context {
            Context::Value => {
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
                            theme.write_value(
                                &state.current_key,
                                &state.current,
                                writer.by_ref(),
                            )?;
                            writer.write(&[b' '])?;

                            // reset state
                            state.current = String::new();
                            state.current_key = String::new();
                            state.context = Context::Key;
                        }
                        _ => state.current.push(ch),
                    }
                }
            }
            Context::Key => match ch {
                '=' => {
                    theme.write_key(&state.current, writer.by_ref())?;
                    writer.write_all(&[b'='])?;

                    state.current_key = state.current;

                    // reset state
                    state.current = String::new();
                    state.context = Context::Value;
                }
                _ => state.current.push(ch),
            },
        }

        Ok::<_, io::Error>(state)
    })?;

    // write the last value
    theme.write_value(&state.current_key, &state.current, writer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhance_logfmt() {
        let log_row = r#"unimportant=string msg="hello world"}"#;
        let mut writer = Vec::new();

        let theme = Theme::default();

        enhance(&theme, log_row, &mut writer).expect("enhance failed");

        let enhanced =
            String::from_utf8(writer).expect("couldn't convert enhanced log row into string");

        assert!(enhanced.contains("\x1b"));
        assert!(log_row.len() < enhanced.len());
        assert!(
            enhanced.contains("unimportant")
                && enhanced.contains("string")
                && enhanced.contains("msg")
                && enhanced.contains("hello world")
        );
    }
}
