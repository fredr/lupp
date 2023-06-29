use std::io;

use crate::styling::{self, Color};

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
                '{' | '}' | ',' => {
                    styling::write_with_color(Color::White, &ch.to_string(), writer.by_ref())?
                }
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
                    styling::write_with_color(Color::White, &ch.to_string(), writer.by_ref())?;
                    state.context = Context::None
                }
                '[' => {
                    styling::write_with_color(Color::White, &ch.to_string(), writer.by_ref())?;
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
                            styling::write_with_color(Color::Gray, "\"", writer.by_ref())?;
                            styling::write_key(&state.current, writer.by_ref())?;
                            styling::write_with_color(Color::Gray, "\"", writer.by_ref())?;

                            state.current_key = state.current;

                            // reset state
                            state.current = String::new();
                            state.context = Context::None;
                        }
                        ch => writer.write_all(&[ch as u8])?,
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
                            styling::write_with_color(Color::Gray, "\"", writer.by_ref())?;
                            styling::write_value(
                                &state.current_key,
                                &state.current,
                                writer.by_ref(),
                            )?;
                            styling::write_with_color(Color::Gray, "\"", writer.by_ref())?;

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
                    styling::write_with_color(Color::Gray, &state.current, writer.by_ref())?;
                    styling::write_with_color(Color::White, &ch.to_string(), writer.by_ref())?;

                    // reset state
                    state.current_key = String::new();
                    state.current = String::new();
                    state.context = Context::None;
                }
                ch => state.current.push(ch),
            },
            Context::ValueArray => match ch {
                ']' => {
                    styling::write_with_color(Color::Gray, &state.current, writer.by_ref())?;
                    styling::write_with_color(Color::White, &ch.to_string(), writer.by_ref())?;

                    state.current = String::new();
                    state.context = Context::None;
                }
                ',' => {
                    styling::write_with_color(Color::Gray, &state.current, writer.by_ref())?;
                    styling::write_with_color(Color::White, &ch.to_string(), writer.by_ref())?;

                    state.current = String::new();
                }
                ch => state.current.push(ch),
            },
        }
        Ok::<_, io::Error>(state)
    })?;

    Ok(())
}
