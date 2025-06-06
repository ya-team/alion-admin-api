use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 插入 sys_user_role 数据
        let insert_user_role_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO sys_user_role (user_id, role_id)
            VALUES
            ('1', '1'),
            ('2', '2'),
            ('3', '3')
            "#.to_string(),
        );

        db.execute(insert_user_role_stmt).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // 当表被删除时，这些数据自然会被删除，所以这里不需要额外的操作
        Ok(())
    }
}
