//! Guest home booklet card — appliance list glance.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{Card, InfoBanner, ListItem, Pressable, Text};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use crate::content::{ApplianceDevice, AppliancesPayload};

pub fn build_home_card(payload: &AppliancesPayload) -> Surface {
    let mut children = Vec::new();
    if !payload.safety_notice.trim().is_empty() {
        children.push(Component::InfoBanner(
            InfoBanner::new()
                .title(json!("i18n:home.card.safety"))
                .message(json!(payload.safety_notice.clone())),
        ));
    }
    for device in payload
        .devices
        .iter()
        .filter(|d| !d.title.trim().is_empty())
    {
        children.push(device_list_item(device));
    }

    Surface::new(
        Card::new()
            .icon(json!("plug"))
            .title(json!("i18n:home.card.title"))
            .action(json!({
                "type": "openOverlay",
                "presentation": "page",
                "surfaceRender": "explore.detail",
                "args": {
                    "icon": "plug",
                    "title": "i18n:home.card.title"
                }
            }))
            .children(children),
    )
    .with_id("home.card")
}

pub fn device_list_item(device: &ApplianceDevice) -> Component {
    let mut item = ListItem::new().title(json!(device.title.clone()));
    if !device.subtitle.trim().is_empty() {
        item = item.subtitle(json!(device.subtitle.clone()));
    }
    if !device.icon.trim().is_empty() {
        item = item.leading(json!(device.icon.clone()));
    }
    item = item.trailing(json!("chevron-right"));
    Component::Pressable(
        Pressable::new()
            .action(json!({
                "type": "openOverlay",
                "presentation": "page",
                "surfaceRender": "explore.item",
                "args": {
                    "deviceId": device.id,
                    "icon": "plug",
                    "title": device.title
                }
            }))
            .child(Component::ListItem(item)),
    )
}

pub fn devices_list(payload: &AppliancesPayload) -> Vec<Component> {
    let mut children = Vec::new();
    if !payload.safety_notice.trim().is_empty() {
        children.push(Component::InfoBanner(
            InfoBanner::new()
                .title(json!("i18n:home.card.safety"))
                .message(json!(payload.safety_notice.clone())),
        ));
    }
    for device in payload
        .devices
        .iter()
        .filter(|d| !d.title.trim().is_empty())
    {
        children.push(device_list_item(device));
    }
    if children.is_empty() {
        children.push(Component::Text(
            Text::new()
                .text(json!("i18n:home.card.empty.description"))
                .variant(json!("body")),
        ));
    }
    children
}
