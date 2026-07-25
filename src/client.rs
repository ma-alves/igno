use std::time::Instant;

use crate::app::Response;

pub async fn fetch(url: &str) -> Result<Response, String> {
    let start = Instant::now();
    let raw_response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let elapsed = start.elapsed();

    Ok(Response::from_raw(raw_response, elapsed).await?)
}
