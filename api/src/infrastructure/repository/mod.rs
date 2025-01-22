use async_trait::async_trait;
use entity::entities::prelude::*;
use sea_orm::DbErr;

pub mod user;

#[async_trait]
pub trait Repository {
    async fn by_id(&self, id: i32) -> Result<Option<User::Model>, DbErr>;
}
