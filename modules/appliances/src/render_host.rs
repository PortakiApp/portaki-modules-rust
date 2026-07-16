//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Text, TextArea};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use crate::store;

#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let _ = ctx;
    let row = store::load_content().ok().flatten();
    let content_fr = row
        .as_ref()
        .map(|r| r.content_fr.clone())
        .unwrap_or_else(|| {
            json!({
                "safety_notice": "Coupez l'eau et le gaz en cas de fuite.",
                "devices": [
                    {
                        "id": "tv",
                        "icon": "📺",
                        "title": "Télévision",
                        "subtitle": "Salon · Samsung 55\"",
                        "steps": ["Allumez avec la télécommande noire.", "Source HDMI 1 pour l'Apple TV."],
                        "tip": "Télécommande sur le meuble TV."
                    }
                ]
            })
            .to_string()
        });
    let content_en = row
        .as_ref()
        .map(|r| r.content_en.clone())
        .unwrap_or_else(|| {
            json!({
                "safety_notice": "Shut off water and gas if you notice a leak.",
                "devices": [
                    {
                        "id": "tv",
                        "icon": "📺",
                        "title": "Television",
                        "subtitle": "Living room · Samsung 55\"",
                        "steps": ["Use the black remote to power on.", "Select HDMI 1 for Apple TV."],
                        "tip": "Remote is on the TV stand."
                    }
                ]
            })
            .to_string()
        });

    let submit_args = json!({
        "content_fr": content_fr,
        "content_en": content_en,
    });
    let save_action =
        serde_json::to_value(Action::command("appliances", "saveContent", submit_args))
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
                            .name(json!("content_fr"))
                            .label(json!("i18n:host.contentFr.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("content_fr"))
                                    .value(json!(content_fr))
                                    .placeholder(json!("i18n:host.content.placeholder")),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("content_en"))
                            .label(json!("i18n:host.contentEn.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("content_en"))
                                    .value(json!(content_en))
                                    .placeholder(json!("i18n:host.content.placeholder")),
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
