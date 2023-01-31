use crate::{
    core::errors::{self, utils::StorageErrorExt, RouterResponse},
    db::StorageInterface,
    services::ApplicationResponse,
    types::storage,
};

pub async fn read_config(
    store: &dyn StorageInterface,
    key: &str,
) -> RouterResponse<storage::Config> {
    Ok(ApplicationResponse::Json(
        store
            .find_config_by_key_cached(key)
            .await
            .map_err(|err| err.to_not_found_response(errors::ApiErrorResponse::ConfigNotFound))?,
    ))
}