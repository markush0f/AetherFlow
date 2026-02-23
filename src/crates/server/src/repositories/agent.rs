use crate::models::agent::{self, Entity as Agent};
use sea_orm::*;

pub struct Repository;

impl Repository {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<agent::Model>, DbErr> {
        Agent::find().all(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<agent::Model>, DbErr> {
        Agent::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        data: agent::ActiveModel,
    ) -> Result<agent::Model, DbErr> {
        data.insert(db).await
    }
}
