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

    pub async fn request(&self, method: Method, url: &str) -> Result<Response, String> {
        let reqwest_method = match method {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Patch => reqwest::Method::PATCH,
            Method::Delete => reqwest::Method::DELETE,
            Method::Head => reqwest::Method::HEAD,
        };

        let start = Instant::now();
        let raw_response = self
            .inner
            .request(reqwest_method, url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let elapsed = start.elapsed();

        Response::from_raw(raw_response, elapsed).await
    }
}
