//! Structured appliances payload (not TipTap).

use serde::{Deserialize, Serialize};

/// One appliance / device guide entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ApplianceDevice {
    #[serde(default)]
    pub id: String,
    /// Emoji or lucide icon name.
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub steps: Vec<String>,
    #[serde(default)]
    pub tip: String,
    #[serde(default, rename = "manualUrl")]
    pub manual_url: String,
}

/// Locale payload stored in `content_fr` / `content_en`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct AppliancesPayload {
    #[serde(default)]
    pub safety_notice: String,
    #[serde(default)]
    pub devices: Vec<ApplianceDevice>,
}

impl AppliancesPayload {
    pub fn parse(raw: &str) -> Self {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Self::default();
        }
        serde_json::from_str(trimmed).unwrap_or_default()
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn is_empty(&self) -> bool {
        self.safety_notice.trim().is_empty()
            && self
                .devices
                .iter()
                .all(|d| d.title.trim().is_empty() && d.id.trim().is_empty())
    }

    pub fn find_device(&self, device_id: &str) -> Option<&ApplianceDevice> {
        self.devices.iter().find(|d| d.id == device_id)
    }
}

pub fn pick_locale(content_fr: &str, content_en: &str, locale: &str) -> AppliancesPayload {
    let prefer_en = locale.to_ascii_lowercase().starts_with("en");
    let primary = if prefer_en { content_en } else { content_fr };
    let fallback = if prefer_en { content_fr } else { content_en };
    let parsed = AppliancesPayload::parse(primary);
    if parsed.is_empty() {
        AppliancesPayload::parse(fallback)
    } else {
        parsed
    }
}
