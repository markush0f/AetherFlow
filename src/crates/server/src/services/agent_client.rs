use reqwest::Client;

pub struct Service;

impl Service {
    /// Executes a given task on a specific remote agent endpoint.
    /// Implements a Resilience Engineering wrapper holding the complex Backoff mechanism.
    pub async fn execute_task(
        client: &Client,
        endpoint: &str,
        payload: &serde_json::Value,
    ) -> Result<(serde_json::Value, i32), (String, i32)> {
        Self::send_with_retry(client, endpoint, payload).await
    }

    /// Internal helper handling exponential backoff and request isolation.
    async fn send_with_retry(
        client: &Client,
        endpoint: &str,
        payload: &serde_json::Value,
    ) -> Result<(serde_json::Value, i32), (String, i32)> {
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
                        let json = response.json::<serde_json::Value>().await.map_err(|e| {
                            (format!("Failed to parse JSON: {}", e), attempt as i32)
                        })?;
                        return Ok((json, attempt as i32));
                    } else if response.status().is_server_error() && attempt < max_retries {
                        // 5xx internal agent errors, retry up to `max_retries`
                        tracing::warn!(
                            "Agent {} returned 5xx on attempt {}. Retrying...",
                            endpoint,
                            attempt
                        );
                    } else {
                        // 4xx errors (client faults) or max retries reached on 5xx
                        // We abort directly and pass the error info.
                        let status = response.status();
                        let err_body = response
                            .text()
                            .await
                            .unwrap_or_else(|_| "Unknown error".to_string());
                        return Err((
                            format!("Agent returned HTTP {}: {}", status.as_u16(), err_body),
                            attempt as i32,
                        ));
                    }
                }
                Err(e) => {
                    if attempt >= max_retries {
                        return Err((
                            format!(
                                "Failed to reach agent at {} after {} attempts: {}",
                                endpoint, max_retries, e
                            ),
                            attempt as i32,
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

            // Exponential backoff logic applies a growing delay
            // before the next attempt, starting from `base_delay`.
            if attempt < max_retries {
                tokio::time::sleep(base_delay).await;
                base_delay *= 2;
            }
        }

        Err((
            "Failed to execute task: Unexpected state".to_string(),
            max_retries as i32,
        ))
    }
}
