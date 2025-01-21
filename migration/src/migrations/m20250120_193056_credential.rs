use extension::postgres::Type;
use sea_orm::{DeriveActiveEnum, EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};
use serde::{Deserialize, Serialize};

use super::m20250120_192657_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ProviderEnum)
                    .values(AuthProvider::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Credential::Table)
                    .if_not_exists()
                    .col(pk_auto(Credential::Id))
                    .col(integer(Credential::UserId))
                    .col(string(Credential::Secret))
                    .col(
                        ColumnDef::new(Credential::Provider)
                            .enumeration(ProviderEnum, AuthProvider::iter()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name("FK_credential_user_id")
                    // HoopId from users_hoops table is a fk to the hoops table
                    .from(User::Table, User::Id)
                    .to(Credential::Table, Credential::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Credential::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Credential {
    Table,
    Id,
    UserId,
    Provider,
    Secret,
}

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    PartialOrd,
    EnumIter,
    DeriveActiveEnum,
    Clone,
    Copy,
    DeriveIden,
)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum AuthProvider {
    #[sea_orm(string_value = "google")]
    Google,
    #[sea_orm(string_value = "discord")]
    Discord,
    #[sea_orm(string_value = "github")]
    Github,
    #[sea_orm(string_value = "password")]
    Password,
}

#[derive(DeriveIden)]
struct ProviderEnum;
