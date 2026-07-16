//! Shared guest SDUI body for access guide.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{Badge, Button, InfoBanner, KeyValue, Link, ListItem, Text};
use serde_json::json;

use super::load::GuestData;

fn kind_label(kind: Option<&str>, locale: &str) -> String {
    let is_en = locale.to_ascii_lowercase().starts_with("en");
    match kind.map(str::trim).unwrap_or("") {
        "parking" => "Parking".to_string(),
        "door" => {
            if is_en {
                "Door".into()
            } else {
                "Porte".into()
            }
        }
        "elevator" => {
            if is_en {
                "Lift".into()
            } else {
                "Ascenseur".into()
            }
        }
        _ => {
            if is_en {
                "Step".into()
            } else {
                "Étape".into()
            }
        }
    }
}

fn external_action(url: &str) -> serde_json::Value {
    serde_json::to_value(Action::External {
        url: url.to_string(),
    })
    .unwrap_or(json!({}))
}

pub fn build_access_glance(data: &GuestData) -> Vec<Component> {
    let mut children = Vec::new();

    if !data.address.is_empty() {
        children.push(Component::KeyValue(
            KeyValue::new()
                .key(json!("i18n:guest.address"))
                .value(json!(data.address.clone())),
        ));
    }
    if !data.gate_code.is_empty() {
        children.push(Component::KeyValue(
            KeyValue::new()
                .key(json!("i18n:guest.gate"))
                .value(json!(data.gate_code.clone())),
        ));
    }
    if !data.keybox_code.is_empty() {
        children.push(Component::KeyValue(
            KeyValue::new()
                .key(json!("i18n:guest.keybox"))
                .value(json!(data.keybox_code.clone())),
        ));
    }
    if !data.parking_info.is_empty() {
        children.push(Component::KeyValue(
            KeyValue::new()
                .key(json!("i18n:guest.parking"))
                .value(json!(data.parking_info.clone())),
        ));
    }

    if !data.parking_map_url.is_empty() {
        children.push(Component::Button(
            Button::new()
                .label(json!("i18n:guest.openMaps"))
                .action(external_action(&data.parking_map_url)),
        ));
    }

    children
}

pub fn build_access_detail(data: &GuestData) -> Vec<Component> {
    let mut children = build_access_glance(data);

    if !data.global_note.is_empty() {
        children.insert(
            0,
            Component::InfoBanner(
                InfoBanner::new()
                    .title(json!("i18n:guest.note.title"))
                    .message(json!(data.global_note.clone())),
            ),
        );
    }

    if !data.arrival_video_url.is_empty() {
        children.push(Component::Link(
            Link::new()
                .label(json!("i18n:guest.watchVideo"))
                .href(json!(data.arrival_video_url.clone()))
                .action(external_action(&data.arrival_video_url)),
        ));
    }

    for step in &data.steps {
        let title = step.title.pick(&data.locale);
        let mut item = ListItem::new()
            .title(json!(title))
            .child(Badge::new().label(json!(kind_label(step.kind.as_deref(), &data.locale))));
        if let Some(detail) = step.detail.as_ref() {
            let text = detail.pick(&data.locale);
            if !text.trim().is_empty() {
                item = item.child(Text::new().text(json!(text)).variant(json!("caption")));
            }
        }
        children.push(Component::ListItem(item));
    }

    children
}
