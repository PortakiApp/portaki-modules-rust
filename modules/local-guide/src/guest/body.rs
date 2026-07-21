//! Shared guest SDUI body for local guide spots.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{InfoBanner, Link, ListItem, Pill, Pressable, Text};
use serde_json::json;

use super::load::GuestData;

pub fn build_spots_body(data: &GuestData, enriched: bool) -> Vec<Component> {
    let mut children = Vec::new();

    if !data.disclaimer.is_empty() {
        children.push(Component::InfoBanner(
            InfoBanner::new()
                .title(json!("i18n:guest.disclaimer.title"))
                .message(json!(data.disclaimer.clone())),
        ));
    }

    for spot in &data.spots {
        let title = spot
            .title
            .pick_with_fallback(&data.locale, &data.property_locale);
        let mut subtitle_parts = Vec::new();
        if let Some(cat) = spot.category.as_deref().filter(|c| !c.trim().is_empty()) {
            subtitle_parts.push(cat.to_string());
        }
        if let Some(dist) = spot.distance.as_deref().filter(|d| !d.trim().is_empty()) {
            subtitle_parts.push(dist.to_string());
        }
        let subtitle = subtitle_parts.join(" · ");

        let mut item = ListItem::new().title(json!(title));
        if !subtitle.is_empty() {
            item = item.subtitle(json!(subtitle));
        }
        if let Some(tag) = spot.tag.as_deref().filter(|t| !t.trim().is_empty()) {
            item = item.child(Pill::new().label(json!(tag)));
        }
        if enriched {
            if let Some(note) = spot.note.as_ref() {
                let text = note.pick_with_fallback(&data.locale, &data.property_locale);
                if !text.trim().is_empty() {
                    item = item.child(Text::new().text(json!(text)).variant(json!("caption")));
                }
            }
            if let Some(detail) = spot.detail.as_ref() {
                let text = detail.pick_with_fallback(&data.locale, &data.property_locale);
                if !text.trim().is_empty() {
                    item = item.child(Text::new().text(json!(text)).variant(json!("body")));
                }
            }
            if let Some(url) = spot.url.as_deref().map(str::trim).filter(|u| !u.is_empty()) {
                let action = serde_json::to_value(Action::External {
                    url: url.to_string(),
                })
                .unwrap_or(json!({}));
                item = item.child(
                    Link::new()
                        .label(json!("i18n:guest.openLink"))
                        .href(json!(url))
                        .action(action),
                );
            }
            children.push(Component::ListItem(item));
        } else if let Some(url) = spot.url.as_deref().map(str::trim).filter(|u| !u.is_empty()) {
            let action = serde_json::to_value(Action::External {
                url: url.to_string(),
            })
            .unwrap_or(json!({}));
            children.push(Component::Pressable(
                Pressable::new().action(action).child(item),
            ));
        } else {
            children.push(Component::ListItem(item));
        }
    }

    children
}
