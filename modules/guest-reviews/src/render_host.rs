//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{
    Button, Field, Form, Page, Select, Text, TextArea, TextInput, Toggle,
};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use crate::config::load_config;

#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let _ = ctx;
    let config = load_config().unwrap_or_default();

    let submit_args = json!({
        "review_channel": config.review_channel.as_str(),
        "show_qr_code": config.show_qr_code,
        "airbnb_review_url": config.airbnb_review_url,
        "thank_you_message": config.thank_you_message,
    });
    let save_action = serde_json::to_value(Action::command(
        "guest-reviews",
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
                            .name(json!("review_channel"))
                            .label(json!("i18n:host.channel.label"))
                            .child(
                                Select::new()
                                    .name(json!("review_channel"))
                                    .options(json!([
                                        {"value": "airbnb", "label": "i18n:host.channel.airbnb"},
                                        {"value": "portaki", "label": "i18n:host.channel.portaki"},
                                        {"value": "both", "label": "i18n:host.channel.both"}
                                    ]))
                                    .value(json!(config.review_channel.as_str())),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("show_qr_code"))
                            .label(json!("i18n:host.qr.label"))
                            .child(
                                Toggle::new()
                                    .name(json!("show_qr_code"))
                                    .checked(json!(config.show_qr_code)),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("airbnb_review_url"))
                            .label(json!("i18n:host.airbnb.label"))
                            .child(
                                TextInput::new()
                                    .name(json!("airbnb_review_url"))
                                    .value(json!(config.airbnb_review_url))
                                    .placeholder(json!("i18n:host.airbnb.placeholder")),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("thank_you_message"))
                            .label(json!("i18n:host.thanks.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("thank_you_message"))
                                    .value(json!(config.thank_you_message))
                                    .placeholder(json!("i18n:host.thanks.placeholder")),
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
