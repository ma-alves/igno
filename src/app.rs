#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestStatus {
    Idle,
    Loading,
}

pub enum ResponseFocus {
    Url,
    Headers,
    Body,
    Response
}

pub struct Request {
    auth: String,
    headers: Vec<(String, String)>,
    url: String,
    method: Method,
}

pub struct Response {
    body: String,
    content_length: String,
    duration: u16,
    headers: Vec<(String, String)>,
    status: Option<u16>,
}

pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Query
}

pub struct App {
    pub url: String,
    pub status: RequestStatus,
    pub request: Option<Request>,
    pub response: Option<Response>,
    pub error: Option<String>,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            url: String::from("https://httpbin.org/get"),
            status: RequestStatus::Idle,
            request: None,
            response: None,
            error: None,
            should_quit: false,
        }
    }
}