/**
 * `SeaORM` Entity for Casbin rules
 * 
 * This module defines the database entity structure for storing Casbin rules.
 * The `Model` struct represents a single row in the `casbin_rule` table.
 */

use sea_orm::entity::prelude::*;

/**
 * Represents a Casbin rule stored in the database
 * 
 * Each rule consists of a policy type (`ptype`) and up to 6 values (`v0` through `v5`).
 * The `ptype` field typically contains either "p" for policy rules or "g" for role rules.
 */
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "casbin_rule")]
pub struct Model {
    /** Primary key of the rule */
    #[sea_orm(primary_key)]
    pub id: i64,
    /** Policy type (e.g., "p" for policy rules, "g" for role rules) */
    pub ptype: String,
    /** First value of the rule */
    pub v0: String,
    /** Second value of the rule */
    pub v1: String,
    /** Third value of the rule */
    pub v2: String,
    /** Fourth value of the rule */
    pub v3: String,
    /** Fifth value of the rule */
    pub v4: String,
    /** Sixth value of the rule */
    pub v5: String,
}

/** Defines the relations between entities */
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
