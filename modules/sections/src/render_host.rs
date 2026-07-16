//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Text, TextArea, TextInput};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use crate::store;

/// Host editor form — create/update one section (workspace tab `sections`).
#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let _ = ctx;
    let sections = store::list_all("fr-FR").unwrap_or_default();
    let (title_fr, body_fr, title_en, body_en) = sections
        .first()
        .map(|section| {
            let fr = section
                .locales
                .iter()
                .find(|l| l.lang.starts_with("fr"));
            let en = section
                .locales
                .iter()
                .find(|l| l.lang.starts_with("en"));
            (
                fr.map(|l| l.title.clone()).unwrap_or_default(),
                fr.map(|l| l.body_markdown.clone()).unwrap_or_default(),
                en.map(|l| l.title.clone()).unwrap_or_default(),
                en.map(|l| l.body_markdown.clone()).unwrap_or_default(),
            )
        })
        .unwrap_or_else(|| {
            (
                "Bienvenue".into(),
                "Bienvenue à L'Islette ! Toute l'équipe vous souhaite un excellent séjour.".into(),
                "Welcome".into(),
                "Welcome to L'Islette! The whole team wishes you a great stay.".into(),
            )
        });

    let section_id = sections.first().map(|s| s.id);
    let submit_args = json!({
        "id": section_id,
        "title_fr": title_fr,
        "title_en": title_en,
        "body_markdown_fr": body_fr,
        "body_markdown_en": body_en,
        "locales": []
    });
    let save_action =
        serde_json::to_value(Action::command("sections", "saveSection", submit_args))
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
                            .name(json!("title_fr"))
                            .label(json!("i18n:host.titleFr.label"))
                            .child(
                                TextInput::new()
                                    .name(json!("title_fr"))
                                    .value(json!(title_fr)),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("body_markdown_fr"))
                            .label(json!("i18n:host.bodyFr.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("body_markdown_fr"))
                                    .value(json!(body_fr)),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("title_en"))
                            .label(json!("i18n:host.titleEn.label"))
                            .child(
                                TextInput::new()
                                    .name(json!("title_en"))
                                    .value(json!(title_en)),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("body_markdown_en"))
                            .label(json!("i18n:host.bodyEn.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("body_markdown_en"))
                                    .value(json!(body_en)),
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
