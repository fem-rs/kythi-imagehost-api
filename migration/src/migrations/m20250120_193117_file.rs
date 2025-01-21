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
                    .table(File::Table)
                    .if_not_exists()
                    .col(pk_auto(File::Id))
                    .col(integer(File::UserId).not_null())
                    .col(string(File::Name).not_null().unique_key())
                    .col(string(File::OriginalName).not_null())
                    .col(string(File::Hash).not_null())
                    .col(integer(File::FileSize).not_null())
                    .col(string(File::FileType).not_null())
                    .col(date(File::UploadedAt).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name("FK_file_user_id")
                    // HoopId from users_hoops table is a fk to the hoops table
                    .from(User::Table, User::Id)
                    .to(File::Table, File::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum File {
    Table,
    Id,
    UserId,
    Name,
    OriginalName,
    Hash,
    FileSize,
    FileType,
    UploadedAt,
}
