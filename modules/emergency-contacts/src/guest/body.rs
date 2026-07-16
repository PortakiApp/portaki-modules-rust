//! Shared guest SDUI body for emergency contacts.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{InfoBanner, Link, ListItem, Pressable, Text};
use serde_json::json;

use super::load::GuestData;

fn tel_action(phone: &str) -> serde_json::Value {
    let tel = phone.replace(|c: char| c.is_whitespace(), "");
    serde_json::to_value(Action::External {
        url: format!("tel:{tel}"),
    })
    .unwrap_or(json!({}))
}

pub fn build_contacts_body(data: &GuestData, show_emergency_banner: bool) -> Vec<Component> {
    let mut children = Vec::new();

    if show_emergency_banner {
        children.push(Component::InfoBanner(
            InfoBanner::new()
                .title(json!("i18n:guest.emergency.title"))
                .message(json!("i18n:guest.emergency.message")),
        ));
    }

    if !data.host_phone.is_empty() {
        let action = tel_action(&data.host_phone);
        children.push(Component::Pressable(
            Pressable::new().action(action.clone()).child(
                ListItem::new()
                    .title(json!("i18n:guest.host.label"))
                    .subtitle(json!(data.host_phone.clone()))
                    .trailing(json!("i18n:guest.call")),
            ),
        ));
    }

    for contact in &data.contacts {
        let label = contact.label.pick(&data.locale);
        let mut item = ListItem::new()
            .title(json!(label))
            .subtitle(json!(contact.phone.clone()))
            .trailing(json!("i18n:guest.call"));
        if let Some(cat) = contact.category.as_deref().filter(|c| !c.trim().is_empty()) {
            item = item.leading(json!(cat));
        }
        if let Some(note) = contact.note.as_ref() {
            let note_text = note.pick(&data.locale);
            if !note_text.trim().is_empty() {
                item = item.child(Text::new().text(json!(note_text)).variant(json!("caption")));
            }
        }
        children.push(Component::Pressable(
            Pressable::new()
                .action(tel_action(&contact.phone))
                .child(item),
        ));
    }

    if show_emergency_banner {
        children.push(Component::Link(
            Link::new()
                .label(json!("i18n:guest.dial112"))
                .href(json!("tel:112"))
                .action(tel_action("112")),
        ));
    }

    children
}
