//! Shared guest SDUI body for facility hours.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{InfoBanner, KeyValue, ListItem, Text};

use super::load::GuestData;

pub fn build_hours_body(data: &GuestData, enriched: bool) -> Vec<Component> {
    let mut children = Vec::new();

    if !data.general_note.is_empty() {
        children.push(Component::InfoBanner(
            InfoBanner::new()
                .title("i18n:guest.note.title")
                .message(data.general_note.clone()),
        ));
    }

    for facility in &data.facilities {
        let title = facility
            .title
            .pick_with_fallback(&data.locale, &data.property_locale);
        let hours = facility
            .hours
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .or_else(|| {
                let joined = facility
                    .lines
                    .iter()
                    .map(|l| l.pick_with_fallback(&data.locale, &data.property_locale))
                    .filter(|s| !s.trim().is_empty())
                    .collect::<Vec<_>>()
                    .join(" · ");
                if joined.is_empty() {
                    None
                } else {
                    Some(joined)
                }
            })
            .unwrap_or_default();

        if enriched {
            let mut item = ListItem::new().title(title);
            if !hours.is_empty() {
                item = item.subtitle(hours.clone());
            }
            for line in &facility.lines {
                let text = line.pick_with_fallback(&data.locale, &data.property_locale);
                if text.trim().is_empty() {
                    continue;
                }
                item = item.child(Text::new().text(text).variant(TextVariant::Caption));
            }
            if let Some(note) = facility.note.as_ref() {
                let note_text = note.pick_with_fallback(&data.locale, &data.property_locale);
                if !note_text.trim().is_empty() {
                    item = item.child(Text::new().text(note_text).variant(TextVariant::Caption));
                }
            }
            children.push(Component::ListItem(item));
        } else {
            children.push(Component::KeyValue(
                KeyValue::new().key(title).value(hours),
            ));
        }
    }

    children
}
