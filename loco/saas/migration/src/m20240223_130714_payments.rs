use loco_rs::schema::timestamptz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Payments::Table)
                    .col(pk_auto(Payments::Id))
                    .col(string(Payments::Email))
                    .col(string(Payments::SubscriptionType))
                    .col(timestamptz(Payments::ExpiresAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Payments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Payments {
    Table,
    Id,
    Email,
    SubscriptionType,
    ExpiresAt,
}
