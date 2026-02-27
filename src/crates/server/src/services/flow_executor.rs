use crate::repositories::{
    flow_execution::Repository as FlowExecutionRepository,
    flow_step::Repository as FlowStepRepository,
};
use crate::services::{
    agent::Service as AgentService, agent_client::Service as AgentClient,
    agent_log::Service as AgentLogService,
};
use reqwest::Client;
use sea_orm::{DatabaseConnection, DbErr};

pub struct Service;

impl Service {
    /// Coordinates the execution pipeline by passing the output of one agent
    /// as the input of the next. Manages flow state and logs via respective services.
    pub async fn execute_flow(
        db: &DatabaseConnection,
        http_client: &Client,
        flow_id: String,
        initial_input: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        // 1. Create a execution record
        let execution =
            FlowExecutionRepository::create(db, flow_id.clone(), Some(initial_input.clone()))
                .await
                .map_err(|e| format!("Failed to create flow execution: {}", e))?;

        let execution_id = execution.id.clone();

        // 2. Fetch the ordered sequence of steps to process
        let steps = FlowStepRepository::get_steps_for_flow(db, flow_id.clone())
            .await
            .map_err(|e| format!("Failed to fetch flow steps: {}", e))?;

        if steps.is_empty() {
            let _ =
                FlowExecutionRepository::update_status(db, execution_id.clone(), "Failed", None)
                    .await;
            return Err("Flow has no steps defined".to_string());
        }

        let mut current_data = initial_input;

        // 3. Sequential operation (Pass the Torch)
        for step in steps {
            // Find the agent in charge of the step
            let agent_opt = AgentService::get_agent_by_id(db, step.agent_id.clone())
                .await
                .map_err(|e| format!("Database error fetching agent: {}", e))?;

            let agent = match agent_opt {
                Some(a) => a,
                None => {
                    let err = format!("Agent {} not found for step", step.agent_id);
                    let _ = FlowExecutionRepository::update_status(
                        db,
                        execution_id.clone(),
                        "Failed",
                        Some(serde_json::json!({ "error": err })),
                    )
                    .await;
                    return Err(err);
                }
            };

            // Prepare payload mixing previous agent output with step static config if set
            let mut payload = current_data.clone();
            if let Some(config) = step.config {
                // If both are objects, merge them. If not, just use current_data. Simple heuristic.
                if let (Some(payload_obj), Some(config_obj)) =
                    (payload.as_object_mut(), config.as_object())
                {
                    for (k, v) in config_obj {
                        payload_obj.insert(k.clone(), v.clone());
                    }
                }
            }

            // Execute the agent and handle retry resilience
            let result = AgentClient::execute_task(http_client, &agent.endpoint, &payload).await;

            let (response_json, retries_used) = match &result {
                Ok((res, retries)) => (res.clone(), *retries),
                Err((err_msg, retries)) => (serde_json::json!({ "error": err_msg }), *retries),
            };

            // Safely log the task in agent_logs
            let ping_db = db.clone();
            let log_agent_id = agent.id.clone();
            let log_payload = payload.clone();
            let log_response = response_json.clone();
            tokio::spawn(async move {
                let _ = AgentLogService::log_task(
                    &ping_db,
                    log_agent_id,
                    log_payload,
                    log_response,
                    retries_used,
                )
                .await;
            });

            // If the execution failed, stop the flow and mark as Failed
            match result {
                Ok((resp, _)) => {
                    current_data = resp;
                }
                Err((e, _)) => {
                    let _ = FlowExecutionRepository::update_status(
                        db,
                        execution_id.clone(),
                        "Failed",
                        Some(serde_json::json!({ "error": e, "step": step.step_order })),
                    )
                    .await;
                    return Err(format!(
                        "Flow failed at step {}, agent {}: {}",
                        step.step_order, agent.slug, e
                    ));
                }
            }
        }

        // 4. Mark execution as completed and store the final output
        let _ = FlowExecutionRepository::update_status(
            db,
            execution_id,
            "Completed",
            Some(current_data.clone()),
        )
        .await;

        Ok(current_data)
    }
}
