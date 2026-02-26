use crate::models;
use crate::services;
use reqwest::Client;
use sea_orm::DatabaseConnection;
use std::time::Duration;
use tracing::info;

pub struct Monitor;

impl Monitor {
    /// Spawns a background Tokio task to periodically ping registered agents
    /// and update their operational status in the database.
    pub fn start_health_check(db: DatabaseConnection, client: Client) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;

                if let Ok(agents) = services::agent::Service::get_all_agents(&db).await {
                    for agent in agents {
                        // Ping the agent's endpoint to check if it's reachable
                        let is_reachable = client.get(&agent.endpoint).send().await.is_ok();
                        let new_status = if is_reachable {
                            models::agent::AgentStatus::Ready
                        } else {
                            models::agent::AgentStatus::Unreachable
                        };

                        // Update database only if the status has actually changed
                        if agent.status != new_status {
                            let _ = services::agent::Service::update_status(
                                &db,
                                agent.id.clone(),
                                new_status.clone(),
                            )
                            .await;
                            info!(
                                "Agent {} ({}) status changed to {:?}",
                                agent.slug, agent.id, new_status
                            );
                        }
                    }
                }
            }
        });
    }
}
