use crate::calc::bit_width::BitWidth;
use crate::calc::engine::CalcResult;
use crate::input::expression::ExpressionState;
use crate::input::real_time::RealTimeState;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    RealTime,
    Expression,
}

pub struct App {
    pub mode: Mode,
    pub bit_width: BitWidth,
    pub running: bool,
    pub show_help: bool,

    pub real_time: RealTimeState,
    pub expression: ExpressionState,
    pub result: Option<CalcResult>,
    pub error: Option<String>,

    pub cursor_visible: bool,
    tick_counter: u64,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: Mode::RealTime,
            bit_width: BitWidth::B32,
            running: true,
            show_help: false,
            real_time: RealTimeState::new(),
            expression: ExpressionState::new(),
            result: None,
            error: None,
            cursor_visible: true,
            tick_counter: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick_counter += 1;
        if self.tick_counter % crate::config::CURSOR_BLINK_INTERVAL == 0 {
            self.cursor_visible = !self.cursor_visible;
        }
    }

    pub fn switch_mode(&mut self) {
        self.mode = match self.mode {
            Mode::RealTime => Mode::Expression,
            Mode::Expression => Mode::RealTime,
        };
        self.error = None;
    }

    pub fn set_bit_width(&mut self, width: BitWidth) {
        self.bit_width = width;
        self.recalculate();
    }

    pub fn recalculate(&mut self) {
        if self.mode == Mode::RealTime {
            self.result = self.real_time.compute(&self.bit_width);
            self.error = self.real_time.error.clone();
        }
    }
}
