//! Host configuration stored in KV (`config` key).

use portaki_sdk::host;
use portaki_sdk::Result;
use serde::{Deserialize, Serialize};

use crate::entities::WeatherUnits;

const CONFIG_KEY: &str = "config";

/// Owner-configurable module settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModuleConfig {
    /// Temperature display unit.
    pub units: WeatherUnits,
    /// Cache refresh cadence label (`1h`, `3h`, `6h`).
    pub refresh_interval: String,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            units: WeatherUnits::Celsius,
            refresh_interval: "1h".to_string(),
        }
    }
}

/// Loads configuration from KV or returns defaults.
pub fn load_config() -> Result<ModuleConfig> {
    let Some(bytes) = host::kv::get(CONFIG_KEY)? else {
        return Ok(ModuleConfig::default());
    };
    serde_json::from_slice(&bytes).map_err(|error| {
        portaki_sdk::PortakiError::Storage(format!("invalid config JSON: {error}"))
    })
}

/// Persists configuration to KV.
pub fn save_config(config: &ModuleConfig) -> Result<()> {
    let bytes = serde_json::to_vec(config).map_err(|error| {
        portaki_sdk::PortakiError::Storage(format!("config serialize: {error}"))
    })?;
    host::kv::set(CONFIG_KEY, &bytes, None)
}
