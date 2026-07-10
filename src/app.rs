use std::time::Duration;

pub struct App {
    pub url: String,
    pub status: RequestStatus,
    pub request: Option<Request>,
    pub response: Option<Response>,
    pub error: Option<String>,
    pub should_quit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestStatus {
    Idle,
    Loading,
}

pub enum ResponseFocus {
    Url,
    Headers,
    Body,
    Response,
}

pub struct Request {
    pub auth: String,
    pub headers: Vec<(String, String)>,
    pub url: String,
    pub method: Method,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub body: String,
    pub duration: u32,
    pub headers: Vec<(String, String)>,
    pub status: Option<u16>,
}

impl Response {
    pub async fn from_raw(raw: reqwest::Response, elapsed: Duration) -> Result<Self, String> {
        let status = Some(raw.status().as_u16());

        let headers = raw
            .headers()
            .iter()
            .map(|(name, value)| {
                (
                    name.to_string(),
                    value.to_str().unwrap_or("<invalid utf8>").to_string(),
                )
            })
            .collect();

        let body = raw.text().await.map_err(|e| e.to_string())?;

        Ok(Self {
            body,
            duration: elapsed.as_millis() as u32,
            headers,
            status,
        })
    }
}

pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Query,
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
