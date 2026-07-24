//! Module commands — configuration persistence.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{load_config, save_config, CalendarFeed, ModuleConfig, CALENDAR_SLOTS};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CalendarInput {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateConfigArgs {
    /// Dynamic list from host StepList (`calendars.{i}.url` / `.label` / `.id`).
    #[serde(default)]
    pub calendars: Vec<CalendarInput>,
    /// Legacy flat fields (pre multi-calendar).
    #[serde(default)]
    pub ical_url_primary: String,
    #[serde(default)]
    pub ical_url_secondary: String,
}

#[portaki_sdk::command(name = "updateConfig")]
pub fn update_config(_ctx: Context, args: UpdateConfigArgs) -> Result<()> {
    let existing = load_config().unwrap_or_default();
    let calendars = calendars_from_args(&args);
    save_config(&ModuleConfig {
        calendars,
        ical_url_primary: String::new(),
        last_sync_at: existing.last_sync_at,
        sync_summary: existing.sync_summary,
    })
}

fn calendars_from_args(args: &UpdateConfigArgs) -> Vec<CalendarFeed> {
    if !args.calendars.is_empty() {
        return args
            .calendars
            .iter()
            .take(CALENDAR_SLOTS)
            .enumerate()
            .filter_map(|(index, input)| {
                let url = input.url.trim();
                if url.is_empty() {
                    return None;
                }
                let id = {
                    let trimmed = input.id.trim();
                    if trimmed.is_empty() {
                        format!("cal-{}", index + 1)
                    } else {
                        trimmed.to_string()
                    }
                };
                let label = {
                    let trimmed = input.label.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed.to_string())
                    }
                };
                Some(CalendarFeed {
                    id,
                    url: url.to_string(),
                    label,
                })
            })
            .collect();
    }

    // Legacy primary / secondary fallback.
    let mut out = Vec::new();
    let primary = args.ical_url_primary.trim();
    if !primary.is_empty() {
        out.push(CalendarFeed {
            id: "cal-1".into(),
            url: primary.to_string(),
            label: None,
        });
    }
    let secondary = args.ical_url_secondary.trim();
    if !secondary.is_empty() {
        out.push(CalendarFeed {
            id: "cal-2".into(),
            url: secondary.to_string(),
            label: None,
        });
    }
    out
}
