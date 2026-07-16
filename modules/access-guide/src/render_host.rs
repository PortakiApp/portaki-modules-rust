//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Text, TextArea, TextInput};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use crate::config::load_config;

#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let _ = ctx;
    let config = load_config().unwrap_or_default();

    let submit_args = json!({
        "steps_json": config.steps_json,
        "parking_map_url": config.parking_map_url,
        "arrival_video_url": config.arrival_video_url,
        "global_note": config.global_note,
        "address": config.address,
        "gate_code": config.gate_code,
        "keybox_code": config.keybox_code,
        "parking_info": config.parking_info,
    });
    let save_action =
        serde_json::to_value(Action::command("access-guide", "updateConfig", submit_args))
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
                            .name(json!("address"))
                            .label(json!("i18n:host.address.label"))
                            .child(
                                TextInput::new()
                                    .name(json!("address"))
                                    .value(json!(config.address)),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("gate_code"))
                            .label(json!("i18n:host.gate.label"))
                            .child(
                                TextInput::new()
                                    .name(json!("gate_code"))
                                    .value(json!(config.gate_code)),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("keybox_code"))
                            .label(json!("i18n:host.keybox.label"))
                            .child(
                                TextInput::new()
                                    .name(json!("keybox_code"))
                                    .value(json!(config.keybox_code)),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("parking_info"))
                            .label(json!("i18n:host.parking.label"))
                            .child(
                                TextInput::new()
                                    .name(json!("parking_info"))
                                    .value(json!(config.parking_info)),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("parking_map_url"))
                            .label(json!("i18n:host.map.label"))
                            .child(
                                TextInput::new()
                                    .name(json!("parking_map_url"))
                                    .value(json!(config.parking_map_url)),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("arrival_video_url"))
                            .label(json!("i18n:host.video.label"))
                            .child(
                                TextInput::new()
                                    .name(json!("arrival_video_url"))
                                    .value(json!(config.arrival_video_url)),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("global_note"))
                            .label(json!("i18n:host.note.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("global_note"))
                                    .value(json!(config.global_note)),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("steps_json"))
                            .label(json!("i18n:host.steps.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("steps_json"))
                                    .value(json!(config.steps_json))
                                    .placeholder(json!("i18n:host.steps.placeholder")),
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
