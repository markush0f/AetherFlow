use crate::models::flow::FlowWithSteps;
use crate::models::flow_step::FlowStepWithAgent;
use crate::models::{agent, flow, flow_step};
use crate::repositories::{
    flow::Repository as FlowRepository, flow_step::Repository as FlowStepRepository,
};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

pub struct Service;

impl Service {
    pub async fn create_flow(
        db: &DatabaseConnection,
        name: String,
        description: Option<String>,
    ) -> Result<flow::Model, DbErr> {
        FlowRepository::create(db, name, description).await
    }

    pub async fn get_all_flows_with_steps(
        db: &DatabaseConnection,
    ) -> Result<Vec<FlowWithSteps>, DbErr> {
        let flows = FlowRepository::find_all(db).await?;
        let flow_ids: Vec<String> = flows.iter().map(|f| f.id.clone()).collect();

        let all_steps = flow_step::Entity::find()
            .filter(flow_step::Column::FlowId.is_in(flow_ids))
            .find_also_related(agent::Entity)
            .all(db)
            .await?;

        let mut result = Vec::new();
        for flow in flows {
            let mut steps_for_flow: Vec<_> = all_steps
                .iter()
                .filter(|(step, _)| step.flow_id == flow.id)
                .cloned()
                .collect();

            steps_for_flow.sort_by_key(|(step, _)| step.step_order);

            let mut agents_chain = Vec::new();
            let steps = steps_for_flow
                .into_iter()
                .map(|(step, opt_agent)| {
                    let slug = opt_agent
                        .map(|a| a.slug)
                        .unwrap_or_else(|| "Unknown Agent".to_string());
                    agents_chain.push(slug.clone());
                    FlowStepWithAgent {
                        step,
                        agent_slug: slug,
                    }
                })
                .collect();

            result.push(FlowWithSteps {
                flow,
                steps,
                agents_chain,
            });
        }

        Ok(result)
    }

    pub async fn get_flow_by_id(
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<flow::Model>, DbErr> {
        FlowRepository::find_by_id(db, id).await
    }

    pub async fn add_flow_step(
        db: &DatabaseConnection,
        flow_id: String,
        agent_id: String,
        step_order: i32,
        config: Option<serde_json::Value>,
    ) -> Result<flow_step::Model, DbErr> {
        FlowStepRepository::create(db, flow_id, agent_id, step_order, config).await
    }

    pub async fn get_flow_steps(
        db: &DatabaseConnection,
        flow_id: String,
    ) -> Result<Vec<flow_step::Model>, DbErr> {
        FlowStepRepository::get_steps_for_flow(db, flow_id).await
    }
}
