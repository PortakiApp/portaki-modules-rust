//! Host configuration stored in KV (`config` key).

use portaki_sdk::host;
use portaki_sdk::Result;
use serde::{Deserialize, Serialize};

const CONFIG_KEY: &str = "config";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ModuleConfig {
    #[serde(default)]
    pub steps_json: String,
    #[serde(default)]
    pub parking_map_url: String,
    #[serde(default)]
    pub arrival_video_url: String,
    #[serde(default)]
    pub global_note: String,
    /// Design glance fields (optional — shown as KeyValue rows).
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub gate_code: String,
    #[serde(default)]
    pub keybox_code: String,
    #[serde(default)]
    pub parking_info: String,
}

impl ModuleConfig {
    pub fn is_empty(&self) -> bool {
        self.parse_steps().is_empty()
            && self.parking_map_url.trim().is_empty()
            && self.arrival_video_url.trim().is_empty()
            && self.global_note.trim().is_empty()
            && self.address.trim().is_empty()
            && self.gate_code.trim().is_empty()
            && self.keybox_code.trim().is_empty()
            && self.parking_info.trim().is_empty()
    }

    pub fn parse_steps(&self) -> Vec<AccessStep> {
        let raw = self.steps_json.trim();
        if raw.is_empty() {
            return Vec::new();
        }
        let Ok(data) = serde_json::from_str::<Vec<AccessStep>>(raw) else {
            return Vec::new();
        };
        data.into_iter()
            .filter(|s| !s.id.trim().is_empty())
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccessStep {
    pub id: String,
    #[serde(default)]
    pub kind: Option<String>,
    pub title: Localized,
    #[serde(default)]
    pub detail: Option<Localized>,
}

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
