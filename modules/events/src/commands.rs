//! Module commands — configuration persistence.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{load_config, save_config, EventRow, Localized, ModuleConfig};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventInput {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub place: String,
    #[serde(default)]
    pub starts_at: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub lat: String,
    #[serde(default)]
    pub lng: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigArgs {
    #[serde(default)]
    pub events: Vec<EventInput>,
    #[serde(default)]
    pub disclaimer: String,
}

#[portaki_sdk::command(name = "updateConfig")]
pub fn update_config(ctx: Context, args: UpdateConfigArgs) -> Result<()> {
    let lang = Localized::lang_code(&ctx.locale);
    let existing = load_config().unwrap_or_default();
    let events = resolve_events(&args.events, &existing.events, &lang);
    let mut disclaimer = existing.disclaimer;
    disclaimer.set(&lang, args.disclaimer.trim().to_string());
    save_config(&ModuleConfig { events, disclaimer })
}

fn resolve_events(args: &[EventInput], existing: &[EventRow], lang: &str) -> Vec<EventRow> {
    args.iter()
        .enumerate()
        .filter_map(|(index, input)| merge_event(input, existing.get(index), index, lang))
        .collect()
}

fn merge_event(
    input: &EventInput,
    previous: Option<&EventRow>,
    index: usize,
    lang: &str,
) -> Option<EventRow> {
    let title_raw = input.title.trim();
    if title_raw.is_empty() {
        return None;
    }

    let mut title = previous.map(|p| p.title.clone()).unwrap_or_default();
    title.set(lang, title_raw.to_string());

    let mut place = previous.map(|p| p.place.clone()).unwrap_or_default();
    place.set(lang, input.place.trim().to_string());

    Some(EventRow {
        id: previous
            .map(|p| p.id.clone())
            .unwrap_or_else(|| format!("evt-{}", index + 1)),
        title,
        place,
        starts_at: input.starts_at.trim().to_string(),
        ends_at: previous.and_then(|p| p.ends_at.clone()),
        url: nonempty_opt(&input.url),
        lat: parse_coord(&input.lat),
        lng: parse_coord(&input.lng),
        note: previous.and_then(|p| p.note.clone()),
    })
}

fn parse_coord(raw: &str) -> Option<f64> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<f64>().ok()
}

fn nonempty_opt(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
