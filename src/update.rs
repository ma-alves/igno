use crate::app::{App, RequestStatus};
use crate::message::Message;

pub fn update(app: &mut App, message: Message) -> Option<Message> {
    match message {
        Message::Char(c) => app.url.push(c),
        Message::Backspace => {
            app.url.pop();
        }
        Message::Quit => app.running = false,
        Message::SendRequest => {
            if app.status == RequestStatus::Loading || app.url.is_empty() {
                return None;
            }
            app.status = RequestStatus::Loading;
            app.error = None;
        }
        Message::ResponseReceived(result) => {
            app.status = RequestStatus::Idle;
            app.pending = false;
            match result {
                Ok(response) => {
                    app.response = Some(response);
                }
                Err(e) => app.error = Some(e),
            }
        }
    };
    None
}
