//! Host configuration stored in KV (`config` key).

use portaki_sdk::host;
use portaki_sdk::Result;
use serde::{Deserialize, Serialize};

use crate::localized::deserialize_localized_field;

pub use crate::localized::Localized;

const CONFIG_KEY: &str = "config";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReviewChannel {
    #[default]
    Airbnb,
    Portaki,
    Both,
}

impl ReviewChannel {
    pub fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "portaki" => Self::Portaki,
            "both" => Self::Both,
            _ => Self::Airbnb,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Airbnb => "airbnb",
            Self::Portaki => "portaki",
            Self::Both => "both",
        }
    }

    pub fn show_airbnb(&self) -> bool {
        matches!(self, Self::Airbnb | Self::Both)
    }

    pub fn show_portaki(&self) -> bool {
        matches!(self, Self::Portaki | Self::Both)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModuleConfig {
    #[serde(default)]
    pub review_channel: ReviewChannel,
    #[serde(default = "default_true")]
    pub show_qr_code: bool,
    #[serde(default)]
    pub airbnb_review_url: String,
    #[serde(default, deserialize_with = "deserialize_localized_field")]
    pub thank_you_message: Localized,
}

fn default_true() -> bool {
    true
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            review_channel: ReviewChannel::Airbnb,
            show_qr_code: true,
            airbnb_review_url: String::new(),
            thank_you_message: Localized::default(),
        }
    }
}

impl ModuleConfig {
    pub fn is_empty(&self) -> bool {
        let airbnb_ok = self.review_channel.show_airbnb() && self.airbnb_url().is_some();
        let portaki_ok = self.review_channel.show_portaki();
        !airbnb_ok && !portaki_ok
    }

    pub fn airbnb_url(&self) -> Option<String> {
        normalize_url(&self.airbnb_review_url)
    }
}

pub fn normalize_url(raw: &str) -> Option<String> {
    let t = raw.trim();
    if t.is_empty() {
        return None;
    }
    let with_scheme = if t.starts_with("http://") || t.starts_with("https://") {
        t.to_string()
    } else {
        format!("https://{t}")
    };
    if with_scheme.starts_with("http://") || with_scheme.starts_with("https://") {
        Some(with_scheme)
    } else {
        None
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
