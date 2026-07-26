use crate::app::{App, Focus, RequestStatus};
use crate::message::Message;

pub enum Command {
    Fetch { url: String },
    None,
}

pub fn update(app: &mut App, message: Message) -> Command {
    match message {
        Message::ToggleFocus => {
            app.focus = match app.focus {
                Focus::RequestFocus => Focus::ResponseFocus,
                Focus::ResponseFocus => Focus::RequestFocus,
            };
            Command::None
        }
        Message::ScrollUp if app.focus == Focus::ResponseFocus => {
            app.response_scroll = app.response_scroll.saturating_sub(1);
            Command::None
        }
        Message::ScrollDown if app.focus == Focus::ResponseFocus => {
            app.response_scroll = app.response_scroll.saturating_add(1);
            Command::None
        }
        Message::PageUp if app.focus == Focus::ResponseFocus => {
            app.response_scroll = app.response_scroll.saturating_sub(10);
            Command::None
        }
        Message::PageDown if app.focus == Focus::ResponseFocus => {
            app.response_scroll = app.response_scroll.saturating_add(10);
            Command::None
        }
        Message::Char(c) => {
            app.url.push(c);
            Command::None
        }
        Message::Backspace => {
            app.url.pop();
            Command::None
        }
        Message::Quit => {
            app.running = false;
            Command::None
        }
        Message::SendRequest => {
            if app.status == RequestStatus::Loading || app.url.is_empty() {
                return Command::None;
            }
            app.status = RequestStatus::Loading;
            app.error = None;
            Command::Fetch {
                url: app.url.clone(),
            }
        }
        Message::ResponseReceived(result) => {
            app.status = RequestStatus::Idle;
            app.response_scroll = 0;
            match result {
                Ok(response) => app.response = Some(response),
                Err(e) => app.error = Some(e),
            }
            Command::None
        }
        _ => Command::None,
    }
}
