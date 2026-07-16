//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Text, TextArea};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use crate::config::load_config;

#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let _ = ctx;
    let config = load_config().unwrap_or_default();

    let submit_args = json!({
        "facilities_json": config.facilities_json,
        "general_note": config.general_note,
    });
    let save_action = serde_json::to_value(Action::command(
        "facility-hours",
        "updateConfig",
        submit_args,
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
                            .name(json!("facilities_json"))
                            .label(json!("i18n:host.facilities.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("facilities_json"))
                                    .value(json!(config.facilities_json))
                                    .placeholder(json!("i18n:host.facilities.placeholder")),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("general_note"))
                            .label(json!("i18n:host.note.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("general_note"))
                                    .value(json!(config.general_note))
                                    .placeholder(json!("i18n:host.note.placeholder")),
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
