use crate::calc::expr_parser;
use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

pub struct ExpressionState {
    pub input: String,
}

impl ExpressionState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }
}

pub fn handle(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => {
            if app.expression.input.is_empty() {
                return;
            }
            match expr_parser::parse(&app.expression.input) {
                Ok(value) => {
                    app.result = Some(crate::calc::engine::CalcResult {
                        value,
                        overflow: false,
                    });
                    app.error = None;
                }
                Err(e) => {
                    app.result = None;
                    app.error = Some(e);
                }
            }
            return;
        }
        KeyCode::Esc => {
            app.expression.input.clear();
            app.result = None;
            app.error = None;
            return;
        }
        KeyCode::Backspace => {
            app.expression.input.pop();
            return;
        }
        KeyCode::Char(c) => {
            app.expression.input.push(c);
            return;
        }
        _ => {}
    }
}
