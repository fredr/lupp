use crate::styling;

enum Context {
    Key,
    Value,
}

struct State {
    line: String,
    current: String,
    current_key: String,
    context: Context,
    escaped: bool,
    quoted: bool,
}

impl State {
    fn new() -> Self {
        Self {
            line: String::new(),
            current: String::new(),
            current_key: String::new(),
            context: Context::Key,
            escaped: false,
            quoted: false,
        }
    }
}

pub fn handle(line: &str) -> String {
    let mut state = line.chars().fold(State::new(), |mut state, ch| {
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
                            styling::write_value(
                                &state.current_key,
                                &state.current,
                                &mut state.line,
                            );
                            state.line.push(' ');

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
                    styling::write_key(&state.current, &mut state.line);
                    state.line.push('=');

                    state.current_key = state.current;

                    // reset state
                    state.current = String::new();
                    state.context = Context::Value;
                }
                _ => state.current.push(ch),
            },
        }

        state
    });

    // write the last value
    styling::write_value(&state.current_key, &state.current, &mut state.line);

    state.line
}
