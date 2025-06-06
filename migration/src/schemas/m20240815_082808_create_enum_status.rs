use sea_orm::EnumIter;
use sea_orm_migration::{
    prelude::{sea_query::extension::postgres::Type, *},
    sea_orm::{ConnectionTrait, DbBackend},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {},
            DbBackend::Postgres => {
                // Create status enum
                manager
                    .create_type(
                        Type::create()
                            .as_enum(Alias::new("status"))
                            .values([Status::Enabled, Status::Disabled, Status::Banned])
                            .to_owned(),
                    )
                    .await?;

                // Create menu_type enum
                manager
                    .create_type(
                        Type::create()
                            .as_enum(Alias::new("menu_type"))
                            .values([MenuType::Directory, MenuType::Menu])
                            .to_owned(),
                    )
                    .await?;
            },
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {},
            DbBackend::Postgres => {
                // Drop status enum
                manager
                    .drop_type(Type::drop().name(Alias::new("status")).to_owned())
                    .await?;
                // Drop menu_type enum
                manager
                    .drop_type(Type::drop().name(Alias::new("menu_type")).to_owned())
                    .await?;
            },
        }

        Ok(())
    }
}

#[derive(DeriveIden, EnumIter)]
pub enum Status {
    #[sea_orm(iden = "status")]
    Enum,
    #[sea_orm(iden = "enabled")]
    Enabled,
    #[sea_orm(iden = "disabled")]
    Disabled,
    #[sea_orm(iden = "banned")]
    Banned,
}

#[derive(DeriveIden, EnumIter)]
pub enum MenuType {
    #[sea_orm(iden = "menu_type")]
    Enum,
    #[sea_orm(iden = "directory")]
    Directory,
    #[sea_orm(iden = "menu")]
    Menu,
}
