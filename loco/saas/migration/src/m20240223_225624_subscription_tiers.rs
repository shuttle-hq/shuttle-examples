use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(SubscriptionTiers::Table)
                    .col(pk_auto(SubscriptionTiers::Id))
                    .col(string(SubscriptionTiers::Tier))
                    .col(string(SubscriptionTiers::StripeItemId))
                    .col(string(SubscriptionTiers::StripePriceId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SubscriptionTiers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SubscriptionTiers {
    Table,
    Id,
    Tier,
    StripeItemId,
    StripePriceId,
}
