use server_global::project_info;
use server_service::admin::{SysAccessKeyService, TAccessKeyService};

pub async fn initialize_access_key() {
    let access_key_service = SysAccessKeyService;

    let _ = access_key_service.initialize_access_key().await;

    project_info!("Access key initialization completed successfully")
}
