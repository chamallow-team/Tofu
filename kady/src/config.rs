use std::path::Path;
use serde::{Deserialize, Serialize};
use twilight_model::gateway::Intents;
use crate::errors::AppError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AppConfig {
    /// The intents of the bot
    ///
    /// See [Twilight - Intents](https://docs.rs/twilight-gateway/latest/twilight_gateway/struct.Intents.html)
    pub intents: Intents,
    /// The configuration for the cache
    pub cache: AppCacheConfig
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AppCacheConfig {
    /// The number of messages stored per channel
    #[serde(default)]
    pub message_cache_size: usize
}

/// Gets the config file and return an error if there is
pub(crate) fn get_config(asset_folder_path: &Path) -> Result<AppConfig, AppError> {
    let content = std::fs::read_to_string(asset_folder_path.join("config.toml"))?;

    toml::from_str(&content)
        .map_err(|err| AppError::ConfigParsing(err))
}