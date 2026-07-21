//! Host dashboard surfaces.
//!
//! Declares workspace tab pathSegment `checklist` via host surface `main`
//! (dashboard resolves module id when `hostSurfaces` pathSegment matches).

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Text, TextInput};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use crate::labels::{self, lang_code};
use crate::storage;

const ITEM_SLOTS: usize = 6;

/// Host checklist editor — indexed item slots → `replaceItems` for active locale.
#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let lang = lang_code(&ctx.locale);
    let items = storage::list_items().unwrap_or_default();

    let submit_items: Vec<serde_json::Value> = items
        .iter()
        .map(|item| {
            json!({
                "label": labels::get_label(item, &lang),
                "sort_order": item.sort_order,
            })
        })
        .collect();

    let save_action = serde_json::to_value(Action::command(
        "checklist",
        "replaceItems",
        json!({ "items": submit_items }),
    ))
    .unwrap_or(json!({}));

    let mut form_children: Vec<Component> = Vec::new();
    for index in 0..ITEM_SLOTS {
        let item = items.get(index);
        let slot = index + 1;
        let label = item
            .map(|i| labels::get_label(i, &lang))
            .unwrap_or_default();

        form_children.push(
            Text::new()
                .text(json!(format!("i18n:host.item.slot{slot}")))
                .variant(json!("caption"))
                .into(),
        );
        form_children.push(
            Field::new()
                .name(json!(format!("items.{index}.label")))
                .label(json!("i18n:host.item.label"))
                .child(
                    TextInput::new()
                        .name(json!(format!("items.{index}.label")))
                        .value(json!(label)),
                )
                .into(),
        );
    }

    form_children.push(
        Text::new()
            .text(json!("i18n:host.main.help"))
            .variant(json!("caption"))
            .into(),
    );
    form_children.push(
        Button::new()
            .label(json!("i18n:host.save"))
            .action(save_action)
            .into(),
    );

    Surface::new(
        Page::new()
            .title(json!("i18n:surface.host.main.title"))
            .child(
                Text::new()
                    .text(json!("i18n:surface.host.main.subtitle"))
                    .variant(json!("body")),
            )
            .child(Form::new().children(form_children)),
    )
    .with_id("main")
}
