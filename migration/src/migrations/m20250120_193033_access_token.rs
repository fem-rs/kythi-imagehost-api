use sea_orm_migration::{prelude::*, schema::*};

use super::m20250120_192657_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AccessToken::Table)
                    .if_not_exists()
                    .col(pk_auto(AccessToken::Id))
                    .col(integer(AccessToken::UserId).not_null())
                    .col(string(AccessToken::Name).not_null())
                    .col(string(AccessToken::Description))
                    .col(string(AccessToken::Token).unique_key().not_null())
                    .col(string(AccessToken::Access))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name("FK_access_token_user_id")
                    // HoopId from users_hoops table is a fk to the hoops table
                    .from(User::Table, User::Id)
                    .to(AccessToken::Table, AccessToken::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccessToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AccessToken {
    Table,
    Id,
    UserId,
    Name,
    Description,
    Token,
    Access,
}
