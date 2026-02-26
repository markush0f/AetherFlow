use reqwest::Client;
use std::time::Duration;

pub struct Service;

impl Service {
    pub async fn execute_task(endpoint: &str, payload: &str) -> Result<String, String> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

        let res = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(payload.to_string())
            .send()
            .await
            .map_err(|e| format!("Failed to reach agent at {}: {}", endpoint, e))?;

        if res.status().is_success() {
            res.text()
                .await
                .map_err(|e| format!("Failed to read agent response body: {}", e))
        } else {
            let status = res.status();
            let err_body = res
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error body".to_string());
            Err(format!(
                "Agent returned HTTP {}: {}",
                status.as_u16(),
                err_body
            ))
        }
    }
}
