use crate::models::flow::{self, Entity as Flow};
use sea_orm::*;

pub struct Repository;

impl Repository {
    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        description: Option<String>,
    ) -> Result<flow::Model, DbErr> {
        let new_flow = flow::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            name: Set(name),
            description: Set(description),
            created_at: Set(None),
            updated_at: Set(None),
        };
        new_flow.insert(db).await
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<flow::Model>, DbErr> {
        Flow::find()
            .order_by_desc(flow::Column::CreatedAt)
            .all(db)
            .await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<flow::Model>, DbErr> {
        Flow::find_by_id(id).one(db).await
    }
}
