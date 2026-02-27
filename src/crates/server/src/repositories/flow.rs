use crate::models::flow::{self, Entity as Flow};
use sea_orm::*;

pub struct Repository;

impl Repository {
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<flow::Model>, DbErr> {
        Flow::find_by_id(id).one(db).await
    }
}
