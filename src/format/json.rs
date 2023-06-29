use std::io;

use crate::styling;

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

pub fn enhance(line: &str, writer: &mut impl io::Write) -> io::Result<()> {
    line.chars().try_fold(State::new(), |mut state, ch| {
        match state.context {
            Context::None => match ch {
                '{' | '}' | ',' => styling::write_highlighted(&ch.to_string().as_str(), writer)?,
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
                    styling::write_highlighted(&ch.to_string().as_str(), writer)?;
                    state.context = Context::None
                }
                '[' => {
                    styling::write_highlighted(&ch.to_string().as_str(), writer)?;
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
                            styling::write_dimmed("\"", writer)?;
                            styling::write_key(&state.current, writer)?;
                            styling::write_dimmed("\"", writer)?;

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
                            styling::write_dimmed("\"", writer)?;
                            styling::write_value(&state.current_key, &state.current, writer)?;
                            styling::write_dimmed("\"", writer)?;

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
                    styling::write_value(&state.current_key, &state.current, writer)?;
                    styling::write_highlighted(&ch.to_string(), writer)?;

                    // reset state
                    state.current_key = String::new();
                    state.current = String::new();
                    state.context = Context::None;
                }
                ch => state.current.push(ch),
            },
            Context::ValueArray => match ch {
                ']' => {
                    styling::write_dimmed(&state.current, writer)?;
                    styling::write_highlighted(&ch.to_string(), writer)?;

                    state.current = String::new();
                    state.context = Context::None;
                }
                ',' => {
                    styling::write_dimmed(&state.current, writer)?;
                    styling::write_highlighted(&ch.to_string(), writer)?;

                    state.current = String::new();
                }
                ch => state.current.push(ch),
            },
        }
        Ok::<_, io::Error>(state)
    })?;

    Ok(())
}
