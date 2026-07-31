use crate::app::{App, Focus, RequestField, RequestStatus};
use crate::client::Method;
use crate::message::Message;

pub enum Command {
    Fetch {
        url: String,
        method: Method,
        body: Option<String>,
        headers: Vec<(String, String)>,
    },
    None,
}

fn active_text_field(app: &mut App) -> Option<&mut String> {
    if app.focus != Focus::RequestFocus {
        return None;
    }
    match app.request_field {
        RequestField::Url => Some(&mut app.url),
        RequestField::Body => Some(&mut app.body),
        RequestField::Headers => Some(&mut app.headers),
        _ => None,
    }
}

fn request_cycle(app: &mut App, forward: bool) {
    match app.request_field {
        RequestField::Method => {
            app.method = if forward { app.method.next() } else { app.method.prev() }
        }
        RequestField::Auth => {
            app.auth = if forward { app.auth.next() } else { app.auth.prev() }
        }
        _ => {}
    }
}

fn parse_headers(raw: &str) -> Vec<(String, String)> {
    raw.lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let (name, value) = line.split_once(':')?;
            Some((name.trim().to_string(), value.trim().to_string()))
        })
        .collect()
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
        Message::SelectNextField => {
            app.request_field = app.request_field.next();
            Command::None
        }
        Message::SelectPrevField => {
            app.request_field = app.request_field.prev();
            Command::None
        }
        Message::CycleRight => {
            request_cycle(app, true);
            Command::None
        }
        Message::CycleLeft => {
            request_cycle(app, false);
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
            if let Some(field) = active_text_field(app) {
                field.push(c);
            }
            Command::None
        }
        Message::Backspace => {
            if let Some(field) = active_text_field(app) {
                field.pop();
            }
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
            let body = (!app.body.is_empty()).then(|| app.body.clone());
            Command::Fetch {
                url: app.url.clone(),
                method: app.method,
                body,
                headers: parse_headers(&app.headers),
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
