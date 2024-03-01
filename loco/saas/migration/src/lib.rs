#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20240223_130714_payments;
mod m20240223_225624_subscription_tiers;
mod m20240224_103401_user_subscriptions;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20240223_130714_payments::Migration),
            Box::new(m20240223_225624_subscription_tiers::Migration),
            Box::new(m20240224_103401_user_subscriptions::Migration),
        ]
    }
}
