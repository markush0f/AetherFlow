use reqwest::Client;

pub struct Service;

impl Service {
    pub async fn execute_task(
        client: &Client,
        endpoint: &str,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let res = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .json(payload)
            .send()
            .await
            .map_err(|e| format!("Failed to reach agent at {}: {}", endpoint, e))?;

        if res.status().is_success() {
            res.json::<serde_json::Value>()
                .await
                .map_err(|e| format!("Failed to parse agent response as JSON: {}", e))
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
