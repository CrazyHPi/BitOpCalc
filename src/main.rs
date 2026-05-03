mod app;
mod calc;
mod config;
mod event;
mod input;
mod ui;

use std::io::{self, stdout};

use app::App;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::EventLoop;
use ratatui::{backend::CrosstermBackend, Terminal};

fn setup_terminal() -> io::Result<()> {
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn restore_terminal() {
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture);
}

fn main() -> io::Result<()> {
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_terminal();
        panic_hook(info);
    }));

    // 尝试设置终端，如果失败则继续运行（可选）
    if let Err(e) = setup_terminal() {
        eprintln!("Warning: Failed to setup terminal: {}", e);
        // 可以选择返回错误或继续运行
        return Err(e);
    }

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new();
    let event_loop = EventLoop::new(config::TICK_RATE);

    let result = loop {
        if !app.running {
            break Ok(());
        }

        if let Err(e) = terminal.draw(|frame| ui::draw(frame, &mut app)) {
            break Err(e);
        }

        match event_loop.next() {
            event::AppEvent::Key(key) => {
                input::handle(&mut app, key);
            }
            event::AppEvent::Tick => {
                app.tick();
            }
            event::AppEvent::Resize => {}
        }
    };

    restore_terminal();

    result
}