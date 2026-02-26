use reqwest::Client;

pub struct Service;

impl Service {
    pub async fn execute_task(
        client: &Client,
        endpoint: &str,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        Self::send_with_retry(client, endpoint, payload).await
    }

    async fn send_with_retry(
        client: &Client,
        endpoint: &str,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let max_retries = 3;
        let mut base_delay = std::time::Duration::from_millis(500);

        for attempt in 1..=max_retries {
            let res = client
                .post(endpoint)
                .header("Content-Type", "application/json")
                .json(payload)
                .send()
                .await;

            match res {
                Ok(response) => {
                    if response.status().is_success() {
                        return response
                            .json::<serde_json::Value>()
                            .await
                            .map_err(|e| format!("Failed to parse JSON: {}", e));
                    } else if response.status().is_server_error() && attempt < max_retries {
                        tracing::warn!(
                            "Agent {} returned 5xx on attempt {}. Retrying...",
                            endpoint,
                            attempt
                        );
                    } else {
                        // 4xx errors or max retries reached on 5xx
                        let status = response.status();
                        let err_body = response
                            .text()
                            .await
                            .unwrap_or_else(|_| "Unknown error".to_string());
                        return Err(format!(
                            "Agent returned HTTP {}: {}",
                            status.as_u16(),
                            err_body
                        ));
                    }
                }
                Err(e) => {
                    if attempt >= max_retries {
                        return Err(format!(
                            "Failed to reach agent at {} after {} attempts: {}",
                            endpoint, max_retries, e
                        ));
                    }
                    tracing::warn!(
                        "Network error reaching agent {} on attempt {}: {}. Retrying...",
                        endpoint,
                        attempt,
                        e
                    );
                }
            }

            // Exponential backoff
            if attempt < max_retries {
                tokio::time::sleep(base_delay).await;
                base_delay *= 2;
            }
        }

        Err("Failed to execute task: Unexpected state".to_string())
    }
}
