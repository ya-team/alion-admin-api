use std::error::Error;

use axum_casbin::CasbinAxumLayer;
use casbin::DefaultModel;
use sea_orm::Database;
use sea_orm_adapter::SeaOrmAdapter;

use crate::project_info;

pub async fn initialize_casbin(
    model_path: &str,
    db_url: &str,
) -> Result<CasbinAxumLayer, Box<dyn Error>> {
    project_info!("Initializing Casbin with model: {}", model_path);
    let model = DefaultModel::from_file(model_path).await?;
    let db = Database::connect(db_url).await?;
    let adapter = SeaOrmAdapter::new(db).await?;

    let casbin_axum_layer = CasbinAxumLayer::new(model, adapter).await?;
    project_info!("Casbin initialization completed successfully");
    Ok(casbin_axum_layer)
}
