use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    Resize,
}

pub struct EventLoop {
    tick_rate: Duration,
}

impl EventLoop {
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_rate_ms),
        }
    }

    pub fn next(&self) -> AppEvent {
        loop {
            if event::poll(self.tick_rate).unwrap() {
                match event::read().unwrap() {
                    Event::Key(key) => {
                        if key.kind == KeyEventKind::Release {
                            continue;
                        }
                        return AppEvent::Key(key);
                    }
                    Event::Resize(_, _) => return AppEvent::Resize,
                    _ => {}
                }
            } else {
                return AppEvent::Tick;
            }
        }
    }
}
