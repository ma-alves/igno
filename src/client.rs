use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}

impl Method {
    pub const ALL: [Method; 6] = [
        Method::Get,
        Method::Post,
        Method::Put,
        Method::Patch,
        Method::Delete,
        Method::Head,
    ];

    pub fn next(self) -> Self {
        Self::ALL[(self as usize + 1) % Self::ALL.len()]
    }

    pub fn prev(self) -> Self {
        Self::ALL[(self as usize + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Patch => "PATCH",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
        };
        f.write_str(s)
    }
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

#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::builder()
                .build()
                .expect("reqwest::Client::builder() should never fail"),
        }
    }

    pub async fn request(
        &self,
        method: Method,
        url: &str,
        body: Option<&str>,
        headers: Vec<(String, String)>,
    ) -> Result<Response, String> {
        let reqwest_method = match method {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Patch => reqwest::Method::PATCH,
            Method::Delete => reqwest::Method::DELETE,
            Method::Head => reqwest::Method::HEAD,
        };

        let mut request = self.inner.request(reqwest_method, url);
        for (name, value) in headers {
            request = request.header(name, value);
        }
        if let Some(body) = body {
            request = request.body(body.to_string());
        }

        let start = Instant::now();
        let raw_response = request.send().await.map_err(|e| e.to_string())?;
        let elapsed = start.elapsed();

        Response::from_raw(raw_response, elapsed).await
    }
}
