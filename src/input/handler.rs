use crossterm::event::KeyEvent;

use crate::app::App;
use crate::config;

pub fn handle(app: &mut App, key: KeyEvent) {
    // Global keys
    match key.code {
        config::KEY_QUIT => {
            app.running = false;
            return;
        }
        config::KEY_SWITCH_MODE => {
            app.switch_mode();
            return;
        }
        config::KEY_HELP => {
            app.show_help = !app.show_help;
            return;
        }
        _ => {}
    }

    // Bit width selection
    if let Some(width) = config::key_to_bit_width(key.code) {
        app.set_bit_width(width);
        return;
    }

    // Mode-specific handling
    match app.mode {
        crate::app::Mode::RealTime => {
            super::real_time::handle(app, key);
        }
        crate::app::Mode::Expression => {
            super::expression::handle(app, key);
        }
    }
}
