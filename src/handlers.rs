use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::App;
use crate::message::Message;

pub fn handle_event(_app: &App) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(50))?
        && let Event::Key(key) = event::read()?
        && key.kind == KeyEventKind::Press
    {
        return Ok(handle_key(key));
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Enter => Some(Message::SendRequest),
        KeyCode::Char(c) => Some(Message::Char(c)),
        KeyCode::Backspace => Some(Message::Backspace),
        _ => None,
    }
}
