use casbin::{error::AdapterError, Error as CasbinError, Filter, Result};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter,
};

use crate::entity::{self, Column, Entity};

const COLUMNS: [Column; 6] = [
    Column::V0,
    Column::V1,
    Column::V2,
    Column::V3,
    Column::V4,
    Column::V5,
];

#[derive(Debug, Default)]
pub(crate) struct Rule<'a> {
    pub(crate) values: [&'a str; 6],
}

impl<'a> Rule<'a> {
    pub(crate) fn from_slice<T: AsRef<str>>(value: &'a [T]) -> Self {
        let mut values = [""; 6];
        for (i, v) in value.iter().enumerate().take(6) {
            values[i] = v.as_ref();
        }
        Rule { values }
    }
}

#[derive(Debug, Default)]
pub(crate) struct RuleWithType<'a> {
    pub(crate) ptype: &'a str,
    pub(crate) rule: Rule<'a>,
}

impl<'a> RuleWithType<'a> {
    pub(crate) fn from_rule(ptype: &'a str, rule: Rule<'a>) -> Self {
        RuleWithType { ptype, rule }
    }
}

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

pub(crate) async fn remove_policies<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    rules: Vec<RuleWithType<'rule>>,
) -> Result<bool> {
    for rule in rules {
        remove_policy(conn, rule).await?;
    }
    Ok(true)
}

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

pub(crate) async fn load_policy<C: ConnectionTrait>(conn: &C) -> Result<Vec<entity::Model>> {
    entity::Entity::find()
        .all(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))
}

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

pub(crate) async fn save_policies<'conn, 'rule, C: ConnectionTrait>(
    conn: &'conn C,
    rules: Vec<RuleWithType<'rule>>,
) -> Result<()> {
    clear_policy(conn).await?;
    add_policies(conn, rules).await?;
    Ok(())
}

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

pub(crate) async fn clear_policy<C: ConnectionTrait>(conn: &C) -> Result<()> {
    Entity::delete_many()
        .exec(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(err))))?;
    Ok(())
}
