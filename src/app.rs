use crate::client::{Method, Response};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    RequestFocus,
    ResponseFocus,
}

pub struct App {
    pub url: String,
    pub status: RequestStatus,
    pub request: Option<Request>,
    pub response: Option<Response>,
    pub error: Option<String>,
    pub running: bool,
    pub response_scroll: u16,
    pub focus: Focus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestStatus {
    Idle,
    Loading,
}

pub struct Request {
    pub auth: String,
    pub headers: Vec<(String, String)>,
    pub url: String,
    pub method: Method,
}

impl Default for App {
    fn default() -> Self {
        Self {
            url: String::from("https://httpbin.org/get"),
            status: RequestStatus::Idle,
            request: None,
            response: None,
            error: None,
            running: true,
            response_scroll: 0,
            focus: Focus::RequestFocus,
        }
    }
}
