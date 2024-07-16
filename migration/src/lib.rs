#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20240716_010639_articles;
mod m20240716_012350_comments;
mod m20240716_014028_movies;
mod m20240716_014124_users_votes;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20240716_010639_articles::Migration),
            Box::new(m20240716_012350_comments::Migration),
            Box::new(m20240716_014028_movies::Migration),
            Box::new(m20240716_014124_users_votes::Migration),
        ]
    }
}