use crate::calc::bit_width::BitWidth;
use crate::calc::engine::{apply, CalcResult, Operator};
use crate::calc::parse_number;
use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OperandField {
    A,
    Operator,
    B,
}

pub struct RealTimeState {
    pub operand_a: String,
    pub operand_b: String,
    pub operator: Operator,
    pub focused_field: OperandField,
    pub error: Option<String>,
}

impl RealTimeState {
    pub fn new() -> Self {
        Self {
            operand_a: String::new(),
            operand_b: String::new(),
            operator: Operator::And,
            focused_field: OperandField::A,
            error: None,
        }
    }

    pub fn compute(&self, width: &BitWidth) -> Option<CalcResult> {
        let a = parse_number(&self.operand_a).unwrap_or(0);
        let b = parse_number(&self.operand_b).unwrap_or(0);

        if self.operand_a.is_empty() && self.operand_b.is_empty() {
            return None;
        }

        Some(apply(self.operator, a, b, width))
    }

    pub fn get_operand_values(&self, width: &BitWidth) -> Option<(u64, u64)> {
        if self.operand_a.is_empty() && self.operand_b.is_empty() {
            return None;
        }
        let mask = width.mask();
        let a = parse_number(&self.operand_a).unwrap_or(0) & mask;
        let b = parse_number(&self.operand_b).unwrap_or(0) & mask;
        Some((a, b))
    }
}

pub fn handle(app: &mut App, key: KeyEvent) {
    let state = &mut app.real_time;

    match key.code {
        KeyCode::Left => {
            state.focused_field = match state.focused_field {
                OperandField::B => OperandField::Operator,
                OperandField::Operator => OperandField::A,
                OperandField::A => OperandField::A,
            };
            return;
        }
        KeyCode::Right => {
            state.focused_field = match state.focused_field {
                OperandField::A => OperandField::Operator,
                OperandField::Operator => OperandField::B,
                OperandField::B => OperandField::B,
            };
            return;
        }
        KeyCode::Up => {
            state.operator = cycle_operator_prev(state.operator);
            app.recalculate();
            return;
        }
        KeyCode::Down => {
            state.operator = cycle_operator_next(state.operator);
            app.recalculate();
            return;
        }
        KeyCode::Backspace => {
            let target = match state.focused_field {
                OperandField::A => &mut state.operand_a,
                OperandField::B => &mut state.operand_b,
                OperandField::Operator => {
                    state.operator = cycle_operator_prev(state.operator);
                    app.recalculate();
                    return;
                }
            };
            target.pop();
            app.recalculate();
            return;
        }
        KeyCode::Enter => {
            // Move to next field
            state.focused_field = match state.focused_field {
                OperandField::A => OperandField::Operator,
                OperandField::Operator => OperandField::B,
                OperandField::B => OperandField::B,
            };
            return;
        }
        KeyCode::Tab => return, // handled globally
        KeyCode::Char(c) => {
            let valid = c.is_ascii_hexdigit() || c == 'x' || c == 'b' || c == '.';
            if !valid {
                return;
            }
            match state.focused_field {
                OperandField::A => state.operand_a.push(c),
                OperandField::B => state.operand_b.push(c),
                OperandField::Operator => {
                    match c {
                        '&' => state.operator = Operator::And,
                        '|' => state.operator = Operator::Or,
                        '^' => state.operator = Operator::Xor,
                        '~' => state.operator = Operator::Not,
                        '<' => state.operator = Operator::ShiftLeft,
                        '>' => state.operator = Operator::ShiftRight,
                        _ => {}
                    }
                    app.recalculate();
                    return;
                }
            }
            app.recalculate();
            return;
        }
        _ => {}
    }
}

fn cycle_operator_next(op: Operator) -> Operator {
    match op {
        Operator::And => Operator::Or,
        Operator::Or => Operator::Xor,
        Operator::Xor => Operator::Not,
        Operator::Not => Operator::ShiftLeft,
        Operator::ShiftLeft => Operator::ShiftRight,
        Operator::ShiftRight => Operator::And,
    }
}

fn cycle_operator_prev(op: Operator) -> Operator {
    match op {
        Operator::And => Operator::ShiftRight,
        Operator::Or => Operator::And,
        Operator::Xor => Operator::Or,
        Operator::Not => Operator::Xor,
        Operator::ShiftLeft => Operator::Not,
        Operator::ShiftRight => Operator::ShiftLeft,
    }
}
