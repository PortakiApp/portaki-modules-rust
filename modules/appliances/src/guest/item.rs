//! Guest explore item — appliance how-to detail.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{InfoBanner, Stack, Text};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use crate::content::{ApplianceDevice, AppliancesPayload};

pub fn build_item_detail(payload: &AppliancesPayload, device_id: Option<&str>) -> Surface {
    let device = device_id
        .and_then(|id| payload.find_device(id))
        .or_else(|| payload.devices.iter().find(|d| !d.title.trim().is_empty()));

    let Some(device) = device else {
        return Surface::new(
            Stack::new().child(
                Text::new()
                    .text(json!("i18n:home.card.empty.description"))
                    .variant(json!("body")),
            ),
        )
        .with_id("explore.item");
    };

    Surface::new(
        Stack::new()
            .gap(json!(12))
            .children(device_detail_children(device)),
    )
    .with_id("explore.item")
}

fn device_detail_children(device: &ApplianceDevice) -> Vec<Component> {
    let mut children = vec![Component::Text(
        Text::new()
            .text(json!(device.title.clone()))
            .variant(json!("title")),
    )];
    if !device.subtitle.trim().is_empty() {
        children.push(Component::Text(
            Text::new()
                .text(json!(device.subtitle.clone()))
                .variant(json!("caption")),
        ));
    }
    children.push(Component::Text(
        Text::new()
            .text(json!("i18n:explore.item.howto"))
            .variant(json!("caption")),
    ));
    for (index, step) in device.steps.iter().enumerate() {
        if step.trim().is_empty() {
            continue;
        }
        children.push(Component::Text(
            Text::new()
                .text(json!(format!("{}. {step}", index + 1)))
                .variant(json!("body")),
        ));
    }
    if !device.tip.trim().is_empty() {
        children.push(Component::InfoBanner(
            InfoBanner::new()
                .title(json!("i18n:explore.item.tip"))
                .message(json!(device.tip.clone())),
        ));
    }
    children
}
