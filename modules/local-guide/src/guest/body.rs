//! Shared guest SDUI body for local guide spots.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{InfoBanner, Link, ListItem, Pill, Pressable, Text};

use super::load::GuestData;

pub fn build_spots_body(data: &GuestData, enriched: bool) -> Vec<Component> {
    let mut children = Vec::new();

    if !data.disclaimer.is_empty() {
        children.push(Component::InfoBanner(
            InfoBanner::new()
                .title("i18n:guest.disclaimer.title")
                .message(data.disclaimer.clone()),
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

        let mut item = ListItem::new().title(title);
        if !subtitle.is_empty() {
            item = item.subtitle(subtitle);
        }
        if let Some(tag) = spot.tag.as_deref().filter(|t| !t.trim().is_empty()) {
            item = item.child(Pill::new().label(tag));
        }
        if enriched {
            if let Some(note) = spot.note.as_ref() {
                let text = note.pick_with_fallback(&data.locale, &data.property_locale);
                if !text.trim().is_empty() {
                    item = item.child(Text::new().text(text).variant(TextVariant::Caption));
                }
            }
            if let Some(detail) = spot.detail.as_ref() {
                let text = detail.pick_with_fallback(&data.locale, &data.property_locale);
                if !text.trim().is_empty() {
                    item = item.child(Text::new().text(text).variant(TextVariant::Body));
                }
            }
            if let Some(url) = spot.url.as_deref().map(str::trim).filter(|u| !u.is_empty()) {
                let action = Action::External {
                    url: url.to_string(),
                };
                item = item.child(
                    Link::new()
                        .label("i18n:guest.openLink")
                        .href(url)
                        .action(action),
                );
            }
            children.push(Component::ListItem(item));
        } else if let Some(url) = spot.url.as_deref().map(str::trim).filter(|u| !u.is_empty()) {
            let action = Action::External {
                url: url.to_string(),
            };
            children.push(Component::Pressable(
                Pressable::new().action(action).child(item),
            ));
        } else {
            children.push(Component::ListItem(item));
        }
    }

    children
}
