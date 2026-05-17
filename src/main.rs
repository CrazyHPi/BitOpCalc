mod app;
mod calc;
mod config;
mod event;
mod input;
mod ui;

use std::io::{self, stdout};
use std::sync::atomic::{AtomicBool, Ordering};

use app::App;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::EventLoop;
use ratatui::{backend::CrosstermBackend, Terminal};

static TERMINAL_SETUP: AtomicBool = AtomicBool::new(false);

#[cfg(windows)]
fn init_windows_console() -> io::Result<()> {
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode,
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE,
    };

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle.is_null() {
            return Err(io::Error::last_os_error());
        }
        let mut mode: u32 = 0;
        if GetConsoleMode(handle as _, &mut mode) == 0 {
            return Err(io::Error::last_os_error());
        }
        SetConsoleMode(handle as _, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    }
    Ok(())
}

fn setup_terminal() -> io::Result<()> {
    #[cfg(windows)]
    init_windows_console()?;

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, DisableMouseCapture)?;
    TERMINAL_SETUP.store(true, Ordering::SeqCst);
    Ok(())
}

fn restore_terminal() {
    if !TERMINAL_SETUP.load(Ordering::SeqCst) {
        return;
    }
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture);
}

fn main() -> io::Result<()> {
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_terminal();
        panic_hook(info);
    }));

    if let Err(e) = setup_terminal() {
        eprintln!("Warning: Failed to setup terminal: {}", e);
    }

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

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
