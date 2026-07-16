//! Structured house-rules payload (not TipTap).

use serde::{Deserialize, Serialize};

/// One rule row shown in the guest booklet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RuleItem {
    /// Lucide / design icon name (`clock-circle`, `x`, …).
    #[serde(default)]
    pub icon: String,
    /// Short rule title.
    #[serde(default)]
    pub title: String,
    /// Optional supporting line.
    #[serde(default)]
    pub subtitle: String,
}

/// Locale payload stored in `content_fr` / `content_en`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RulesPayload {
    #[serde(default)]
    pub items: Vec<RuleItem>,
}

impl RulesPayload {
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
        self.items.iter().all(|item| item.title.trim().is_empty())
    }
}

/// Picks FR or EN payload from a content row.
pub fn pick_locale(content_fr: &str, content_en: &str, locale: &str) -> RulesPayload {
    let prefer_en = locale.to_ascii_lowercase().starts_with("en");
    let primary = if prefer_en { content_en } else { content_fr };
    let fallback = if prefer_en { content_fr } else { content_en };
    let parsed = RulesPayload::parse(primary);
    if parsed.is_empty() {
        RulesPayload::parse(fallback)
    } else {
        parsed
    }
}
