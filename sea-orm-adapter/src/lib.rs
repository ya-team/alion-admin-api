/**
 * SeaORM adapter for Casbin
 * 
 * This crate provides a SeaORM adapter for Casbin, allowing you to store Casbin policies in a database
 * using SeaORM. It supports various database backends including PostgreSQL, MySQL, and SQLite.
 * 
 * # Example
 * ```rust
 * use sea_orm::Database;
 * use sea_orm_adapter::SeaOrmAdapter;
 * 
 * async fn example() {
 *     let db = Database::connect("postgres://user:pass@localhost/db").await.unwrap();
 *     let adapter = SeaOrmAdapter::new(db).await.unwrap();
 * }
 * ```
 */

pub use adapter::SeaOrmAdapter;
pub use migration::{down, up};

mod action;
mod adapter;
pub mod entity;
mod migration;
