use crate::repositories::StorageRepositories;

pub mod record;
pub mod repositories;
pub mod utils;

pub async fn initialize_repositories() -> anyhow::Result<StorageRepositories> {
    let pool = utils::local_data::initialize_local_db().await?;
    Ok(StorageRepositories::new(pool))
}
