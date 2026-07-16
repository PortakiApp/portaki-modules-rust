//! Host configuration stored in KV (`config` key).

use portaki_sdk::host;
use portaki_sdk::Result;
use serde::{Deserialize, Serialize};

const CONFIG_KEY: &str = "config";

/// Owner-configurable module settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ModuleConfig {
    /// JSON array of bins (`id`, `title` FR/EN, `items`, optional `color`).
    #[serde(default)]
    pub bins_json: String,
    /// Free-text collection schedule.
    #[serde(default)]
    pub collection_schedule: String,
}

impl ModuleConfig {
    /// True when neither bins nor schedule are configured.
    pub fn is_empty(&self) -> bool {
        self.parse_bins().is_empty() && self.collection_schedule.trim().is_empty()
    }

    /// Parses `bins_json` into typed rows (invalid JSON → empty).
    pub fn parse_bins(&self) -> Vec<BinRow> {
        let raw = self.bins_json.trim();
        if raw.is_empty() {
            return Vec::new();
        }
        let Ok(data) = serde_json::from_str::<Vec<BinRow>>(raw) else {
            return Vec::new();
        };
        data.into_iter()
            .filter(|b| !b.id.trim().is_empty())
            .collect()
    }
}

/// One recycling bin entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BinRow {
    pub id: String,
    pub title: Localized,
    #[serde(default)]
    pub items: Vec<Localized>,
    /// Optional hex color for the bin chip (e.g. `#f4c020`).
    #[serde(default)]
    pub color: Option<String>,
}

/// FR/EN localized string pair.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Localized {
    #[serde(default)]
    pub fr: String,
    #[serde(default)]
    pub en: String,
}

impl Localized {
    pub fn pick(&self, locale: &str) -> String {
        if locale.to_ascii_lowercase().starts_with("en") {
            if !self.en.trim().is_empty() {
                self.en.clone()
            } else {
                self.fr.clone()
            }
        } else if !self.fr.trim().is_empty() {
            self.fr.clone()
        } else {
            self.en.clone()
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
