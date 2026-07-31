use crate::client::{Method, Response};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    RequestFocus,
    ResponseFocus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestField {
    Url,
    Method,
    Auth,
    Body,
    Headers,
}

impl RequestField {
    pub const ALL: [RequestField; 5] = [
        RequestField::Url,
        RequestField::Method,
        RequestField::Auth,
        RequestField::Body,
        RequestField::Headers,
    ];

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|f| *f == self).unwrap()
    }

    pub fn next(self) -> Self {
        Self::ALL[(self.index() + 1) % Self::ALL.len()]
    }

    pub fn prev(self) -> Self {
        Self::ALL[(self.index() + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Auth {
    None,
    Basic,
    Bearer,
}

impl Auth {
    pub const ALL: [Auth; 3] = [Auth::None, Auth::Basic, Auth::Bearer];

    pub fn next(self) -> Self {
        Self::ALL[(self as usize + 1) % Self::ALL.len()]
    }

    pub fn prev(self) -> Self {
        Self::ALL[(self as usize + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

impl std::fmt::Display for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Auth::None => "none",
            Auth::Basic => "basic",
            Auth::Bearer => "bearer",
        };
        f.write_str(s)
    }
}

pub struct App {
    pub url: String,
    pub method: Method,
    pub auth: Auth,
    pub body: String,
    pub headers: String,
    pub request_field: RequestField,
    pub status: RequestStatus,
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

impl Default for App {
    fn default() -> Self {
        Self {
            url: String::from("https://httpbin.org/get"),
            method: Method::Get,
            auth: Auth::None,
            body: String::new(),
            headers: String::new(),
            request_field: RequestField::Url,
            status: RequestStatus::Idle,
            response: None,
            error: None,
            running: true,
            response_scroll: 0,
            focus: Focus::RequestFocus,
        }
    }
}
