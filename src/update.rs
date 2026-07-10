use ratatui::crossterm::event::KeyCode;

use crate::app::{App, RequestStatus};
use crate::command::Command;
use crate::message::Message;

/// Pure: (Message, &mut App) -> Command. No I/O, no async, no channels.
/// Fully unit-testable without spinning up a terminal or a runtime.
pub fn update(message: Message, app: &mut App) -> Command {
    match message {
        Message::KeyPressed(key) => match key.code {
            KeyCode::Char('q') if app.status != RequestStatus::Loading => {
                app.should_quit = true;
                Command::None
            }
            KeyCode::Enter => update(Message::SendRequest, app),
            KeyCode::Char(c) => {
                app.url.push(c);
                Command::None
            }
            KeyCode::Backspace => {
                app.url.pop();
                Command::None
            }
            _ => Command::None,
        },

        Message::SendRequest => {
            if app.status == RequestStatus::Loading || app.url.is_empty() {
                return Command::None;
            }
            app.status = RequestStatus::Loading;
            app.error = None;
            Command::FetchUrl(app.url.clone())
        }

        Message::ResponseReceived(result) => {
            app.status = RequestStatus::Idle;
            match result {
                Ok(result) => {
                    app.response = Some(result);
                    app.error = None;
                }
                Err(e) => app.error = Some(e),
            }
            Command::None
        }

        Message::Quit => {
            app.should_quit = true;
            Command::None
        }
    }
}
