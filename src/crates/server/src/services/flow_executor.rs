use crate::repositories::{
    flow_execution::Repository as FlowExecutionRepository,
    flow_step::Repository as FlowStepRepository,
};
use crate::services::{
    agent::Service as AgentService, agent_client::Service as AgentClient,
    agent_log::Service as AgentLogService,
};
use reqwest::Client;
use sea_orm::{DatabaseConnection, EntityTrait};

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
            // Find the task in charge of the step
            let task_opt = crate::models::agent_task::Entity::find_by_id(step.task_id.clone())
                .one(db)
                .await
                .map_err(|e| format!("Database error fetching task: {}", e))?;

            let task = match task_opt {
                Some(t) => t,
                None => {
                    let err = format!("Task {} not found for step", step.task_id);
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

            let agent_opt = AgentService::get_agent_by_id(db, task.agent_id.clone())
                .await
                .map_err(|e| format!("Database error fetching agent: {}", e))?;

            let agent = match agent_opt {
                Some(a) => a,
                None => {
                    let err = format!("Agent {} not found for task", task.agent_id);
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

            // Prepare payload by dynamically acting as a template engine
            let mut payload = current_data.clone();

            // Build the URL
            let base_url = agent.endpoint.trim_end_matches('/');
            let endpoint = match &task.path {
                Some(p) => {
                    if p.starts_with('/') {
                        format!("{}{}", base_url, p)
                    } else {
                        format!("{}/{}", base_url, p)
                    }
                }
                None => base_url.to_string(),
            };

            if let Some(config) = step.config {
                if let Some(config_obj) = config.as_object() {
                    let mut new_payload = serde_json::Map::new();

                    // Extract a clean string representation of the current data (e.g. previous step output)
                    let input_str = match &current_data {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Object(obj) => {
                            if let Some(content) = obj.get("response").and_then(|v| v.as_str()) {
                                content.to_string()
                            } else if let Some(content) = obj
                                .get("message")
                                .and_then(|v| v.get("content"))
                                .and_then(|v| v.as_str())
                            {
                                content.to_string()
                            } else {
                                current_data.to_string()
                            }
                        }
                        v => v.to_string(),
                    };

                    // 1. Template Interpolation
                    if let Some(template_val) = config_obj.get("template").and_then(|v| v.as_str())
                    {
                        let interpolated = template_val.replace("{{input}}", &input_str);
                        new_payload.insert("prompt".to_string(), serde_json::json!(interpolated));
                    } else if let Some(prompt_val) =
                        config_obj.get("prompt").and_then(|v| v.as_str())
                    {
                        // Fallback: If there's a prompt, also try to interpolate it
                        let interpolated = prompt_val.replace("{{input}}", &input_str);
                        new_payload.insert("prompt".to_string(), serde_json::json!(interpolated));
                    } else {
                        // If no template, inject current_data as raw format
                        new_payload.insert("prompt".to_string(), serde_json::json!(input_str));
                    }

                    // 2. Extract control parameters and map them for Ollama payload construction
                    if let Some(system_prompt) = config_obj.get("system_prompt") {
                        new_payload.insert("system".to_string(), system_prompt.clone());
                    }
                    if let Some(temperature) = config_obj.get("temperature") {
                        // Put inside options for standard Ollama? Or root? We'll put root, standard Ollama API accepts temperature at root
                        new_payload.insert("temperature".to_string(), temperature.clone());
                    }
                    if let Some(model) = config_obj.get("model") {
                        new_payload.insert("model".to_string(), model.clone());
                    }

                    // 3. Keep moving existing objects if they are not the mapped ones
                    for (k, v) in config_obj {
                        if k != "template"
                            && k != "prompt"
                            && k != "system_prompt"
                            && k != "temperature"
                            && k != "model"
                        {
                            new_payload.insert(k.clone(), v.clone());
                        }
                    }

                    // Only map stream: false to avoid streaming chunks response parsing
                    new_payload.insert("stream".to_string(), serde_json::json!(false));

                    payload = serde_json::Value::Object(new_payload);
                }
            }

            // Execute the agent and handle retry resilience
            let result = AgentClient::execute_task(http_client, &endpoint, &payload).await;

            let (response_json, retries_used) = match &result {
                Ok((res, retries)) => (res.clone(), *retries),
                Err((err_msg, retries)) => (serde_json::json!({ "error": err_msg }), *retries),
            };

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
