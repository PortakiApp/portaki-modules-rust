//! Host configuration stored in KV (`config` key).

use portaki_sdk::host;
use portaki_sdk::Result;
use serde::{Deserialize, Serialize};

const CONFIG_KEY: &str = "config";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ModuleConfig {
    #[serde(default)]
    pub smartlock_id: String,
    #[serde(default)]
    pub keypad_code: String,
    #[serde(default)]
    pub device_name: String,
}

impl ModuleConfig {
    pub fn is_empty(&self) -> bool {
        self.smartlock_id.trim().is_empty()
            && self.keypad_code.trim().is_empty()
            && self.device_name.trim().is_empty()
    }

    pub fn keypad_code_trimmed(&self) -> &str {
        self.keypad_code.trim()
    }
}

pub fn load_config() -> Result<ModuleConfig> {
    let Some(bytes) = host::kv::get(CONFIG_KEY)? else {
        return Ok(ModuleConfig::default());
    };
    serde_json::from_slice(&bytes).map_err(|error| {
        portaki_sdk::PortakiError::Storage(format!("invalid config JSON: {error}"))
    })
}

pub fn save_config(config: &ModuleConfig) -> Result<()> {
    let bytes = serde_json::to_vec(config).map_err(|error| {
        portaki_sdk::PortakiError::Storage(format!("config serialize: {error}"))
    })?;
    host::kv::set(CONFIG_KEY, &bytes, None)
}
