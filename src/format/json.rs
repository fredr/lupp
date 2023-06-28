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
    line: String,
    current: String,
    current_key: String,
    context: Context,
    escaped: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            line: String::new(),
            current: String::new(),
            current_key: String::new(),
            context: Context::None,
            escaped: false,
        }
    }
}

pub fn enhance(line: &str) -> String {
    let state = line.chars().fold(State::new(), |mut state, ch| {
        match state.context {
            Context::None => match ch {
                '{' | '}' | ',' => {
                    styling::write_with_color(Color::White, &ch.to_string(), &mut state.line)
                }
                '"' => state.context = Context::Key,
                ':' => {
                    state.line.push(':');
                    state.context = Context::Value
                }
                ch => state.line.push(ch),
            },
            Context::Value => match ch {
                '"' => state.context = Context::ValueString,
                '0'..='9' | 'a'..='z' => {
                    // we don't want to be strict, so treat any unquoted strings as booleans
                    state.current.push(ch);
                    state.context = Context::ValueNumber
                }
                '{' => {
                    styling::write_with_color(Color::White, &ch.to_string(), &mut state.line);
                    state.context = Context::None
                }
                '[' => {
                    styling::write_with_color(Color::White, &ch.to_string(), &mut state.line);
                    state.context = Context::ValueArray
                }
                ch => state.line.push(ch),
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
                            styling::write_with_color(Color::Gray, "\"", &mut state.line);
                            styling::write_key(&state.current, &mut state.line);
                            styling::write_with_color(Color::Gray, "\"", &mut state.line);

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
                            styling::write_with_color(Color::Gray, "\"", &mut state.line);
                            styling::write_value(
                                &state.current_key,
                                &state.current,
                                &mut state.line,
                            );
                            styling::write_with_color(Color::Gray, "\"", &mut state.line);

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
                    styling::write_with_color(Color::Gray, &state.current, &mut state.line);
                    styling::write_with_color(Color::White, &ch.to_string(), &mut state.line);

                    // reset state
                    state.current_key = String::new();
                    state.current = String::new();
                    state.context = Context::None;
                }
                ch => state.current.push(ch),
            },
            Context::ValueArray => match ch {
                ']' => {
                    styling::write_with_color(Color::Gray, &state.current, &mut state.line);
                    styling::write_with_color(Color::White, &ch.to_string(), &mut state.line);

                    state.current = String::new();
                    state.context = Context::None;
                }
                ',' => {
                    styling::write_with_color(Color::Gray, &state.current, &mut state.line);
                    styling::write_with_color(Color::White, &ch.to_string(), &mut state.line);

                    state.current = String::new();
                }
                ch => state.current.push(ch),
            },
        }
        state
    });

    state.line
}
