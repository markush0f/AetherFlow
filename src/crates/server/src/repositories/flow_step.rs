use crate::models::flow_step::{self, Column, Entity as FlowStep};
use sea_orm::{QueryOrder, *};

pub struct Repository;

impl Repository {
    pub async fn get_steps_for_flow(
        db: &DatabaseConnection,
        flow_id: String,
    ) -> Result<Vec<flow_step::Model>, DbErr> {
        FlowStep::find()
            .filter(Column::FlowId.eq(flow_id))
            .order_by_asc(Column::StepOrder)
            .all(db)
            .await
    }
}
