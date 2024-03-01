use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(UserSubscriptions::Table)
                    .col(pk_auto(UserSubscriptions::Id))
                    .col(integer(UserSubscriptions::UserId))
                    .col(string(UserSubscriptions::UserTier))
                    .col(string(UserSubscriptions::StripeCustomerId))
                    .col(string(UserSubscriptions::StripeSubscriptionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_subscriptions-users")
                            .from(UserSubscriptions::Table, UserSubscriptions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserSubscriptions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserSubscriptions {
    Table,
    Id,
    UserId,
    UserTier,
    StripeCustomerId,
    StripeSubscriptionId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
