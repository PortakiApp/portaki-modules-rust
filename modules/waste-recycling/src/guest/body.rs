//! Shared guest SDUI body for waste bins.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{ColorDotItem, InfoBanner, ListItem, Text};
use serde_json::json;

use super::load::GuestData;

/// Glance / detail shared body: bin rows + optional collection banner.
pub fn build_bins_body(data: &GuestData, enriched: bool) -> Vec<Component> {
    let mut children = Vec::new();

    for bin in &data.bins {
        let title = bin
            .title
            .pick_with_fallback(&data.locale, &data.property_locale);
        let subtitle = bin
            .items
            .iter()
            .map(|item| item.pick_with_fallback(&data.locale, &data.property_locale))
            .filter(|s| !s.trim().is_empty())
            .collect::<Vec<_>>()
            .join(", ");

        if let Some(color) = bin.color.as_deref().filter(|c| !c.trim().is_empty()) {
            children.push(Component::ColorDotItem(
                ColorDotItem::new()
                    .label(json!(format!("{title} — {subtitle}")))
                    .color(json!(color)),
            ));
        } else {
            let mut item = ListItem::new().title(json!(title));
            if !subtitle.is_empty() {
                item = item.subtitle(json!(subtitle));
            }
            if enriched {
                for line in &bin.items {
                    let text = line.pick_with_fallback(&data.locale, &data.property_locale);
                    if text.trim().is_empty() {
                        continue;
                    }
                    item = item.child(Text::new().text(json!(text)).variant(json!("caption")));
                }
            }
            children.push(Component::ListItem(item));
        }
    }

    if !data.collection_schedule.is_empty() {
        children.push(Component::InfoBanner(
            InfoBanner::new()
                .title(json!("i18n:guest.collection.title"))
                .message(json!(data.collection_schedule.clone())),
        ));
    }

    children
}
