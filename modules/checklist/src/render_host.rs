//! Host dashboard surfaces.
//!
//! Declares workspace tab pathSegment `checklist` via host surface `main`
//! (dashboard resolves module id when `hostSurfaces` pathSegment matches).

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Text, TextArea};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use crate::storage;

/// Host checklist editor — JSON items → `replaceItems`.
#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let _ = ctx;
    let items = storage::list_items().unwrap_or_default();
    let items_json = serde_json::to_string_pretty(
        &items
            .iter()
            .map(|item| {
                json!({
                    "labelFr": item.label_fr,
                    "labelEn": item.label_en,
                    "sortOrder": item.sort_order,
                })
            })
            .collect::<Vec<_>>(),
    )
    .unwrap_or_else(|_| "[]".to_string());

    let save_action = serde_json::to_value(Action::command(
        "checklist",
        "replaceItems",
        json!({ "itemsJson": items_json }),
    ))
    .unwrap_or(json!({}));

    Surface::new(
        Page::new()
            .title(json!("i18n:surface.host.main.title"))
            .child(
                Text::new()
                    .text(json!("i18n:surface.host.main.subtitle"))
                    .variant(json!("body")),
            )
            .child(
                Form::new()
                    .child(
                        Field::new()
                            .name(json!("itemsJson"))
                            .label(json!("i18n:host.items.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("itemsJson"))
                                    .value(json!(items_json))
                                    .placeholder(json!("i18n:host.items.placeholder")),
                            ),
                    )
                    .child(
                        Text::new()
                            .text(json!("i18n:host.main.help"))
                            .variant(json!("caption")),
                    )
                    .child(
                        Button::new()
                            .label(json!("i18n:host.save"))
                            .action(save_action),
                    ),
            ),
    )
    .with_id("main")
}
