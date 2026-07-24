//! Host configuration stored in KV (`config` key).

use portaki_sdk::host;
use portaki_sdk::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const CONFIG_KEY: &str = "config";

/// Soft cap for host SDUI rows (abuse / UI guard). Not a product “max 2”.
pub const CALENDAR_SLOTS: usize = 20;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CalendarFeed {
    pub id: String,
    #[serde(default)]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl CalendarFeed {
    pub fn trimmed_url(&self) -> Option<&str> {
        trim_url(&self.url)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModuleConfig {
    #[serde(default)]
    pub calendars: Vec<CalendarFeed>,
    /// Mirrored first calendar URL for platform guest-field sync (`property.icalUrl`).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ical_url_primary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_sync_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync_summary: Option<String>,
}

impl ModuleConfig {
    pub fn connected_calendars(&self) -> Vec<&CalendarFeed> {
        self.calendars
            .iter()
            .filter(|c| c.trimmed_url().is_some())
            .collect()
    }

    pub fn has_any_feed(&self) -> bool {
        !self.connected_calendars().is_empty()
    }

    /// Keep `ical_url_primary` aligned with the first connected calendar URL.
    pub fn sync_primary_mirror(&mut self) {
        self.ical_url_primary = self
            .connected_calendars()
            .first()
            .and_then(|c| c.trimmed_url())
            .unwrap_or("")
            .to_string();
    }
}

fn trim_url(raw: &str) -> Option<&str> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Wire format that accepts the calendars list plus legacy primary/secondary / feeds_json.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct RawConfig {
    calendars: Vec<CalendarFeed>,
    ical_url_primary: String,
    ical_url_secondary: String,
    feeds_json: String,
    last_sync_at: Option<String>,
    sync_summary: Option<String>,
}

fn migrate_raw(raw: RawConfig) -> ModuleConfig {
    let mut calendars = raw
        .calendars
        .into_iter()
        .enumerate()
        .filter_map(|(index, mut feed)| {
            let url = feed.url.trim().to_string();
            if url.is_empty() {
                return None;
            }
            feed.url = url;
            if feed.id.trim().is_empty() {
                feed.id = format!("cal-{}", index + 1);
            }
            if let Some(label) = feed.label.take() {
                let trimmed = label.trim().to_string();
                feed.label = if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                };
            }
            Some(feed)
        })
        .collect::<Vec<_>>();

    if calendars.is_empty() {
        calendars = calendars_from_legacy_urls(&raw.ical_url_primary, &raw.ical_url_secondary);
    }
    if calendars.is_empty() {
        calendars = calendars_from_feeds_json(&raw.feeds_json);
    }

    let mut config = ModuleConfig {
        calendars,
        ical_url_primary: String::new(),
        last_sync_at: nonempty_opt(raw.last_sync_at),
        sync_summary: nonempty_opt(raw.sync_summary),
    };
    config.sync_primary_mirror();
    config
}

fn calendars_from_legacy_urls(primary: &str, secondary: &str) -> Vec<CalendarFeed> {
    let mut out = Vec::new();
    if let Some(url) = trim_url(primary) {
        out.push(CalendarFeed {
            id: "cal-1".into(),
            url: url.to_string(),
            label: None,
        });
    }
    if let Some(url) = trim_url(secondary) {
        out.push(CalendarFeed {
            id: "cal-2".into(),
            url: url.to_string(),
            label: None,
        });
    }
    out
}

fn calendars_from_feeds_json(raw: &str) -> Vec<CalendarFeed> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }
    let Ok(values) = serde_json::from_str::<Vec<Value>>(trimmed) else {
        return Vec::new();
    };
    values
        .into_iter()
        .enumerate()
        .filter_map(|(index, value)| {
            let url = value
                .get("url")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())?
                .to_string();
            let id = value
                .get("id")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .unwrap_or_else(|| format!("cal-{}", index + 1));
            let label = value
                .get("label")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string);
            Some(CalendarFeed { id, url, label })
        })
        .collect()
}

fn nonempty_opt(value: Option<String>) -> Option<String> {
    value.and_then(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

pub fn load_config() -> Result<ModuleConfig> {
    let Some(bytes) = host::kv::get(CONFIG_KEY)? else {
        return Ok(ModuleConfig::default());
    };
    let raw: RawConfig = serde_json::from_slice(&bytes).map_err(|error| {
        portaki_sdk::PortakiError::Storage(format!("invalid config JSON: {error}"))
    })?;
    Ok(migrate_raw(raw))
}

pub fn save_config(config: &ModuleConfig) -> Result<()> {
    let mut config = config.clone();
    config.sync_primary_mirror();
    // Drop empty rows before persist.
    config.calendars.retain(|c| c.trimmed_url().is_some());
    let bytes = serde_json::to_vec(&config).map_err(|error| {
        portaki_sdk::PortakiError::Storage(format!("config serialize: {error}"))
    })?;
    host::kv::set(CONFIG_KEY, &bytes, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrates_primary_secondary_urls() {
        let cfg = migrate_raw(RawConfig {
            ical_url_primary: " https://a.ics ".into(),
            ical_url_secondary: "https://b.ics".into(),
            ..RawConfig::default()
        });
        assert_eq!(cfg.calendars.len(), 2);
        assert_eq!(cfg.calendars[0].url, "https://a.ics");
        assert_eq!(cfg.calendars[1].url, "https://b.ics");
        assert_eq!(cfg.ical_url_primary, "https://a.ics");
    }

    #[test]
    fn migrates_feeds_json() {
        let cfg = migrate_raw(RawConfig {
            feeds_json:
                r#"[{"url":"https://x.ics"},{"url":"https://y.ics"},{"url":"https://z.ics"}]"#
                    .into(),
            ..RawConfig::default()
        });
        assert_eq!(cfg.calendars.len(), 3);
        assert_eq!(cfg.ical_url_primary, "https://x.ics");
    }

    #[test]
    fn calendars_list_wins_over_legacy() {
        let cfg = migrate_raw(RawConfig {
            calendars: vec![CalendarFeed {
                id: "c1".into(),
                url: "https://only.ics".into(),
                label: Some("Airbnb".into()),
            }],
            ical_url_primary: "https://legacy.ics".into(),
            ical_url_secondary: "https://legacy2.ics".into(),
            ..RawConfig::default()
        });
        assert_eq!(cfg.calendars.len(), 1);
        assert_eq!(cfg.calendars[0].url, "https://only.ics");
    }

    #[test]
    fn empty_urls_are_ignored() {
        let config = ModuleConfig {
            calendars: vec![
                CalendarFeed {
                    id: "a".into(),
                    url: "  ".into(),
                    label: None,
                },
                CalendarFeed {
                    id: "b".into(),
                    url: "https://example.com/a.ics".into(),
                    label: None,
                },
            ],
            ..Default::default()
        };
        assert_eq!(config.connected_calendars().len(), 1);
        assert!(config.has_any_feed());
    }
}
