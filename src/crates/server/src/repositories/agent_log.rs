use crate::models::agent_log::{self};
use sea_orm::*;

pub struct Repository;

impl Repository {
    pub async fn create(
        db: &DatabaseConnection,
        data: agent_log::ActiveModel,
    ) -> Result<agent_log::Model, DbErr> {
        data.insert(db).await
    }
}
