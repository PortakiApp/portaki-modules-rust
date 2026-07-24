//! Property stats strip — `property-stats-card` host surface.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{Card, Field, Page, Text};
use portaki_sdk::sdui::surface::Surface;

use crate::config::load_config;

#[portaki_sdk::surface(host, id = "calendar-sync")]
pub fn render_host_stats(_ctx: HostContext) -> Surface {
    let config = load_config().unwrap_or_default();
    let connected = config.connected_calendars().len();
    let last_sync = config
        .last_sync_at
        .clone()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "i18n:stats.never".to_string());
    let summary = config
        .sync_summary
        .clone()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "i18n:stats.emptySummary".to_string());
    let failed = summary_mentions_failures(config.sync_summary.as_deref());

    let mut children: Vec<Component> = vec![
        Field::new()
            .name("connected_calendars")
            .label("i18n:stats.connected")
            .child(
                Text::new()
                    .text(format!("{connected}"))
                    .variant(TextVariant::Body),
            )
            .into(),
        Field::new()
            .name("last_sync_at")
            .label("i18n:stats.lastSync")
            .child(Text::new().text(last_sync).variant(TextVariant::Caption))
            .into(),
        Field::new()
            .name("sync_summary")
            .label("i18n:stats.summary")
            .child(Text::new().text(summary).variant(TextVariant::Caption))
            .into(),
    ];

    if failed {
        children.push(
            Text::new()
                .text("i18n:stats.errorsHint")
                .variant(TextVariant::Caption)
                .into(),
        );
    }

    Surface::new(
        Page::new().child(
            Card::new()
                .title("i18n:stats.title")
                .subtitle("i18n:stats.subtitle")
                .icon("calendar")
                .children(children),
        ),
    )
    .with_id(crate::ids::HOST_STATS)
}

fn summary_mentions_failures(summary: Option<&str>) -> bool {
    let Some(raw) = summary.map(str::trim).filter(|s| !s.is_empty()) else {
        return false;
    };
    // Summary format: "N stay(s) · X feed(s) ok · Y feed(s) failed"
    for part in raw.split('·') {
        let part = part.trim();
        if !part.contains("failed") {
            continue;
        }
        let digits: String = part.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(n) = digits.parse::<u32>() {
            return n > 0;
        }
    }
    false
}
