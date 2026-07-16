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
        "spots_json": config.spots_json,
        "disclaimer": config.disclaimer,
    });
    let save_action =
        serde_json::to_value(Action::command("local-guide", "updateConfig", submit_args))
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
                            .name(json!("spots_json"))
                            .label(json!("i18n:host.spots.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("spots_json"))
                                    .value(json!(config.spots_json))
                                    .placeholder(json!("i18n:host.spots.placeholder")),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("disclaimer"))
                            .label(json!("i18n:host.disclaimer.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("disclaimer"))
                                    .value(json!(config.disclaimer))
                                    .placeholder(json!("i18n:host.disclaimer.placeholder")),
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
