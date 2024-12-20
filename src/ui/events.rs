//ui/events.rs
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use std::io::Result;

pub enum AppEvent {
    Input(KeyCode),
    Resize(u16, u16),
    Tick,
}

pub struct Events {
    tick_rate: Duration,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate}
    }

    pub fn next_event(&self) -> Result<AppEvent> {
        // poll for events and if none return Tick after the tick rate passes.
        if event::poll(self.tick_rate)? {
            if let Event::Key(k) = event::read()? {
                return Ok(AppEvent::Input(k.code));
            } else if let Event::Resize(w, h) = event::read()? {
                return Ok(AppEvent::Resize(w, h));
            }
        }
        Ok(AppEvent::Tick)
    }
}