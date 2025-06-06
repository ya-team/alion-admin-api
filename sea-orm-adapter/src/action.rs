/**
 * Core functionality for managing Casbin rules in the database
 * 
 * This module provides the core functions for managing Casbin rules in the database,
 * including adding, removing, and querying rules.
 */

use casbin::{error::AdapterError, Error as CasbinError, Filter, Result};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter,
};

use crate::entity::{self, Column, Entity};

/** Array of column identifiers for rule values */
const COLUMNS: [Column; 6] = [
    Column::V0,
    Column::V1,
    Column::V2,
    Column::V3,
    Column::V4,
    Column::V5,
];

/**
 * Represents a Casbin rule with up to 6 values
 */
#[derive(Debug, Default)]
pub(crate) struct Rule<'a> {
    /** Array of rule values */
    pub(crate) values: [&'a str; 6],
}

impl<'a> Rule<'a> {
    /**
     * Creates a new Rule from a slice of strings
     * 
     * # Arguments
     * * `value` - A slice of strings representing the rule values
     */
    pub(crate) fn from_slice<T: AsRef<str>>(value: &'a [T]) -> Self {
        let mut values = [""; 6];
        for (i, v) in value.iter().enumerate().take(6) {
            values[i] = v.as_ref();
        }
        Rule { values }
    }
}

/**
 * Represents a Casbin rule with its policy type
 */
#[derive(Debug, Default)]
pub(crate) struct RuleWithType<'a> {
    /** The policy type of the rule (e.g., "p" or "g") */
    pub(crate) ptype: &'a str,
    /** The rule values */
    pub(crate) rule: Rule<'a>,
}

impl<'a> RuleWithType<'a> {
    /**
     * Creates a new RuleWithType from a policy type and rule
     * 
     * # Arguments
     * * `ptype` - The policy type
     * * `rule` - The rule values
     */
    pub(crate) fn from_rule(ptype: &'a str, rule: Rule<'a>) -> Self {
        RuleWithType { ptype, rule }
    }
}

/**
 * Removes a single policy rule from the database
 * 
 * # Arguments
 * * `conn` - Database connection
 * * `rule` - The rule to remove
 * 
 * # Returns
 * * `Result<bool>` - True if the rule was removed, false otherwise
 */
pub(crate) async fn remove_policy<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    rule: RuleWithType<'rule>,
) -> Result<bool> {
    let mut delete = Entity::delete_many().filter(Column::Ptype.eq(rule.ptype));
    for (column, value) in COLUMNS.iter().zip(rule.rule.values.iter()) {
        delete = delete.filter(column.eq(*value));
    }
    delete
        .exec(conn)
        .await
        .map(|count| count.rows_affected == 1)
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
}

/**
 * Removes multiple policy rules from the database
 * 
 * # Arguments
 * * `conn` - Database connection
 * * `rules` - Vector of rules to remove
 * 
 * # Returns
 * * `Result<bool>` - True if all rules were removed successfully
 */
pub(crate) async fn remove_policies<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    rules: Vec<RuleWithType<'rule>>,
) -> Result<bool> {
    for rule in rules {
        remove_policy(conn, rule).await?;
    }
    Ok(true)
}

/**
 * Removes filtered policy rules from the database
 * 
 * # Arguments
 * * `conn` - Database connection
 * * `ptype` - Policy type
 * * `index_of_match_start` - Starting index for matching
 * * `rule` - Rule to match against
 * 
 * # Returns
 * * `Result<bool>` - True if any rules were removed
 */
pub(crate) async fn remove_filtered_policy<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    ptype: &'rule str,
    index_of_match_start: usize,
    rule: Rule<'rule>,
) -> Result<bool> {
    let base_condition = Condition::all().add(Column::Ptype.eq(ptype));
    let conditions = rule.values[index_of_match_start..]
        .iter()
        .zip(&COLUMNS[index_of_match_start..])
        .filter(|(value, _)| !value.is_empty())
        .fold(base_condition, |acc, (value, column)| {
            acc.add(column.eq(*value))
        });

    Entity::delete_many()
        .filter(conditions)
        .exec(conn)
        .await
        .map(|count| count.rows_affected >= 1)
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
}

/**
 * Loads all policy rules from the database
 * 
 * # Arguments
 * * `conn` - Database connection
 * 
 * # Returns
 * * `Result<Vec<entity::Model>>` - Vector of policy rules
 */
pub(crate) async fn load_policy<C: ConnectionTrait>(conn: &C) -> Result<Vec<entity::Model>> {
    entity::Entity::find()
        .all(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
}

/**
 * Loads filtered policy rules from the database
 * 
 * # Arguments
 * * `conn` - Database connection
 * * `filter` - Filter criteria
 * 
 * # Returns
 * * `Result<Vec<entity::Model>>` - Vector of filtered policy rules
 */
pub(crate) async fn load_filtered_policy<'conn, 'filter, C: ConnectionTrait>(
    conn: &'conn C,
    filter: Filter<'filter>,
) -> Result<Vec<entity::Model>> {
    let g_filter = Rule::from_slice(&filter.g);
    let p_filter = Rule::from_slice(&filter.p);

    let g_condition = create_condition_from_rule("g", &g_filter);
    let p_condition = create_condition_from_rule("p", &p_filter);

    Entity::find()
        .filter(Condition::any().add(g_condition).add(p_condition))
        .all(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
}

/**
 * Creates a database condition from a rule
 * 
 * # Arguments
 * * `prefix` - Policy type prefix
 * * `rule` - Rule to create condition from
 * 
 * # Returns
 * * `Condition` - Database query condition
 */
fn create_condition_from_rule(prefix: &str, rule: &Rule) -> Condition {
    rule.values
        .iter()
        .zip(COLUMNS.iter())
        .filter(|(value, _)| !value.is_empty())
        .fold(
            Condition::all().add(Column::Ptype.starts_with(prefix)),
            |acc, (value, column)| acc.add(column.eq(*value)),
        )
}

/**
 * Saves all policy rules to the database
 * 
 * # Arguments
 * * `conn` - Database connection
 * * `rules` - Vector of rules to save
 * 
 * # Returns
 * * `Result<()>` - Success or error
 */
pub(crate) async fn save_policies<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    rules: Vec<RuleWithType<'rule>>,
) -> Result<()> {
    clear_policy(conn).await?;
    add_policies(conn, rules).await?;
    Ok(())
}

/**
 * Adds a single policy rule to the database
 * 
 * # Arguments
 * * `conn` - Database connection
 * * `rule` - Rule to add
 * 
 * # Returns
 * * `Result<bool>` - True if the rule was added successfully
 */
pub(crate) async fn add_policy<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    rule: RuleWithType<'rule>,
) -> Result<bool> {
    let model = create_active_model(&rule);
    model
        .insert(conn)
        .await
        .map(|_| true)
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
}

/**
 * Adds multiple policy rules to the database
 * 
 * # Arguments
 * * `conn` - Database connection
 * * `rules` - Vector of rules to add
 * 
 * # Returns
 * * `Result<bool>` - True if all rules were added successfully
 */
pub(crate) async fn add_policies<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    rules: Vec<RuleWithType<'rule>>,
) -> Result<bool> {
    let models: Vec<entity::ActiveModel> = rules.iter().map(create_active_model).collect();
    Entity::insert_many(models)
        .exec(conn)
        .await
        .map(|_| true)
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
}

/**
 * Creates an active model from a rule
 * 
 * # Arguments
 * * `rule` - Rule to create model from
 * 
 * # Returns
 * * `entity::ActiveModel` - Database active model
 */
fn create_active_model(rule: &RuleWithType) -> entity::ActiveModel {
    entity::ActiveModel {
        id: NotSet,
        ptype: Set(rule.ptype.to_string()),
        v0: Set(rule.rule.values[0].to_string()),
        v1: Set(rule.rule.values[1].to_string()),
        v2: Set(rule.rule.values[2].to_string()),
        v3: Set(rule.rule.values[3].to_string()),
        v4: Set(rule.rule.values[4].to_string()),
        v5: Set(rule.rule.values[5].to_string()),
    }
}

/**
 * Clears all policy rules from the database
 * 
 * # Arguments
 * * `conn` - Database connection
 * 
 * # Returns
 * * `Result<()>` - Success or error
 */
pub(crate) async fn clear_policy<C: ConnectionTrait>(conn: &C) -> Result<()> {
    Entity::delete_many()
        .exec(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;
    Ok(())
}
