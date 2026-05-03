use crossterm::event::KeyCode;

use crate::calc::bit_width::BitWidth;

/// Global key bindings
pub const KEY_QUIT: KeyCode = KeyCode::Char('q');
pub const KEY_SWITCH_MODE: KeyCode = KeyCode::Tab;
pub const KEY_HELP: KeyCode = KeyCode::Char('?');

/// Bit width selection: 1->8bit, 2->16bit, 3->32bit, 4->64bit
pub fn key_to_bit_width(key: KeyCode) -> Option<BitWidth> {
    match key {
        KeyCode::Char('1') => Some(BitWidth::B8),
        KeyCode::Char('2') => Some(BitWidth::B16),
        KeyCode::Char('3') => Some(BitWidth::B32),
        KeyCode::Char('4') => Some(BitWidth::B64),
        _ => None,
    }
}

/// Tick rate for the event loop
pub const TICK_RATE: u64 = 100;

/// Cursor blink interval in ticks
pub const CURSOR_BLINK_INTERVAL: u64 = 5;
