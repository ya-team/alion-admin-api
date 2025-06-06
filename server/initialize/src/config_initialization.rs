use crate::{project_error, project_info};

pub async fn initialize_config(file_path: &str) {
    match server_config::init_from_file(file_path).await {
        Ok(_) => {
            project_info!("Configuration initialized successfully from: {}", file_path)
        },
        Err(e) => {
            project_error!("Failed to initialize config from {}: {:?}", file_path, e);
        },
    }
}
