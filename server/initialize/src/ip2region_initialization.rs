use std::error::Error;

use xdb::searcher;

use crate::project_info;

pub async fn init_xdb() -> Result<(), Box<dyn Error>> {
    tokio::task::spawn_blocking(|| {
        searcher::searcher_init(Some("server/resources/ip2region.xdb".to_string()));
    })
    .await?;
    project_info!("XDB initialized successfully");
    Ok(())
}
