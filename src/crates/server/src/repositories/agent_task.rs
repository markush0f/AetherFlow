use crate::models::agent_task::{self, Column, Entity as AgentTask};
use sea_orm::{ColumnTrait, QueryFilter, QueryOrder, *};

pub struct Repository;

impl Repository {
    pub async fn create(
        db: &DatabaseConnection,
        data: agent_task::ActiveModel,
    ) -> Result<agent_task::Model, DbErr> {
        data.insert(db).await
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<agent_task::Model>, DbErr> {
        AgentTask::find().order_by_asc(Column::Name).all(db).await
    }

    pub async fn find_by_agent_id(
        db: &DatabaseConnection,
        agent_id: String,
    ) -> Result<Vec<agent_task::Model>, DbErr> {
        AgentTask::find()
            .filter(Column::AgentId.eq(agent_id))
            .order_by_asc(Column::Name)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<agent_task::Model>, DbErr> {
        AgentTask::find_by_id(id).one(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: String) -> Result<u64, DbErr> {
        let result = AgentTask::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected)
    }
}
