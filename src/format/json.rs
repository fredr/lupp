use std::io;

use crate::styling::Theme;

enum Context {
    None,
    Value,
    ValueString,
    ValueNumber,
    ValueArray,
    Key,
}

struct State {
    current: String,
    current_key: String,
    context: Context,
    escaped: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            current: String::new(),
            current_key: String::new(),
            context: Context::None,
            escaped: false,
        }
    }
}

pub fn enhance(theme: &Theme, line: &str, writer: &mut impl io::Write) -> io::Result<()> {
    line.chars().try_fold(State::new(), |mut state, ch| {
        match state.context {
            Context::None => match ch {
                '{' | '}' | ',' => theme.write_highlighted(&ch.to_string().as_str(), writer)?,
                '"' => state.context = Context::Key,
                ':' => {
                    writer.write_all(&[b':'])?;
                    state.context = Context::Value
                }
                ch => writer.write_all(&[ch as u8])?,
            },
            Context::Value => match ch {
                '"' => state.context = Context::ValueString,
                '0'..='9' | 'a'..='z' => {
                    // we don't want to be strict, so treat any unquoted strings as booleans
                    state.current.push(ch);
                    state.context = Context::ValueNumber
                }
                '{' => {
                    theme.write_highlighted(&ch.to_string().as_str(), writer)?;
                    state.context = Context::None
                }
                '[' => {
                    theme.write_highlighted(&ch.to_string().as_str(), writer)?;
                    state.context = Context::ValueArray
                }
                ch => writer.write_all(&[ch as u8])?,
            },
            Context::Key => {
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
                            theme.write_dimmed("\"", writer)?;
                            theme.write_key(&state.current, writer)?;
                            theme.write_dimmed("\"", writer)?;

                            state.current_key = state.current;

                            // reset state
                            state.current = String::new();
                            state.context = Context::None;
                        }
                        ch => state.current.push(ch),
                    }
                }
            }
            Context::ValueString => {
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
                            theme.write_dimmed("\"", writer)?;
                            theme.write_value(&state.current_key, &state.current, writer)?;
                            theme.write_dimmed("\"", writer)?;

                            // reset state
                            state.current_key = String::new();
                            state.current = String::new();
                            state.context = Context::None;
                        }
                        ch => state.current.push(ch),
                    }
                }
            }
            Context::ValueNumber => match ch {
                ',' | '}' => {
                    theme.write_value(&state.current_key, &state.current, writer)?;
                    theme.write_highlighted(&ch.to_string(), writer)?;

                    // reset state
                    state.current_key = String::new();
                    state.current = String::new();
                    state.context = Context::None;
                }
                ch => state.current.push(ch),
            },
            Context::ValueArray => match ch {
                ']' => {
                    theme.write_dimmed(&state.current, writer)?;
                    theme.write_highlighted(&ch.to_string(), writer)?;

                    state.current = String::new();
                    state.context = Context::None;
                }
                ',' => {
                    theme.write_dimmed(&state.current, writer)?;
                    theme.write_highlighted(&ch.to_string(), writer)?;

                    state.current = String::new();
                }
                ch => state.current.push(ch),
            },
        }
        Ok::<_, io::Error>(state)
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhance_json() {
        let log_row = r#"{"unimportant": "string", "msg": "hello world"}"#;
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
