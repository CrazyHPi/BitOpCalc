mod app;
mod calc;
mod config;
mod event;
mod input;
mod ui;

use std::io;

use app::App;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::EventLoop;
use ratatui::{backend::CrosstermBackend, Terminal};

struct TerminalGuard;

impl TerminalGuard {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
    }
}

fn main() -> io::Result<()> {
    let _guard = TerminalGuard::new()?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new();
    let event_loop = EventLoop::new(config::TICK_RATE);

    loop {
        if !app.running {
            break;
        }

        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        match event_loop.next() {
            event::AppEvent::Key(key) => {
                input::handle(&mut app, key);
            }
            event::AppEvent::Tick => {
                app.tick();
            }
            event::AppEvent::Resize => {}
        }
    }

    Ok(())
}
