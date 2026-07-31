use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::{App, Focus};
use crate::message::Message;

pub fn handle_event(app: &App) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(50))?
        && let Event::Key(key) = event::read()?
        && key.kind == KeyEventKind::Press
    {
        return Ok(handle_key(key, app.focus));
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent, focus: Focus) -> Option<Message> {
    match key.code {
        KeyCode::Tab => Some(Message::ToggleFocus),
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Enter => Some(Message::SendRequest),
        KeyCode::Up => Some(match focus {
            Focus::RequestFocus => Message::SelectPrevField,
            Focus::ResponseFocus => Message::ScrollUp,
        }),
        KeyCode::Down => Some(match focus {
            Focus::RequestFocus => Message::SelectNextField,
            Focus::ResponseFocus => Message::ScrollDown,
        }),
        KeyCode::Left if focus == Focus::RequestFocus => Some(Message::CycleLeft),
        KeyCode::Right if focus == Focus::RequestFocus => Some(Message::CycleRight),
        KeyCode::PageUp => Some(Message::PageUp),
        KeyCode::PageDown => Some(Message::PageDown),
        KeyCode::Char(c) => Some(Message::Char(c)),
        KeyCode::Backspace => Some(Message::Backspace),
        _ => None,
    }
}
