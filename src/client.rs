/// The result of a completed HTTP request.
#[derive(Debug, Clone)]
pub struct ApiResult {
    pub status: u16,
    pub body: String,
}

/// A plain async function with no dependency on Ratatui, `Msg`, or
/// channels. This is what makes it trivial to unit-test on its own
/// or swap out later (e.g. for a mock in tests).
pub async fn fetch(url: &str) -> Result<ApiResult, String> {
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let status = response.status().as_u16();
    let body = response.text().await.map_err(|e| e.to_string())?;
    Ok(ApiResult { status, body })
}