//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Text, TextInput};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use crate::config::{load_config, ContactRow, Localized};

const CONTACT_SLOTS: usize = 6;

#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let lang = Localized::lang_code(&ctx.locale);
    let config = load_config().unwrap_or_default();
    let contacts = config.parse_contacts();

    let submit_args = json!({
        "contacts": contacts_to_submit(&contacts, &lang),
        "host_visible_phone": config.host_visible_phone,
    });
    let save_action = serde_json::to_value(Action::command(
        "emergency-contacts",
        "updateConfig",
        submit_args,
    ))
    .unwrap_or(json!({}));

    let mut form_children: Vec<Component> = vec![Field::new()
        .name(json!("host_visible_phone"))
        .label(json!("i18n:host.phone.label"))
        .child(
            TextInput::new()
                .name(json!("host_visible_phone"))
                .value(json!(config.host_visible_phone))
                .placeholder(json!("i18n:host.phone.placeholder")),
        )
        .into()];

    for index in 0..CONTACT_SLOTS {
        push_contact_slot(&mut form_children, index, contacts.get(index), &lang);
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

fn contacts_to_submit(contacts: &[ContactRow], lang: &str) -> Vec<serde_json::Value> {
    contacts
        .iter()
        .map(|c| {
            json!({
                "label": c.label.get(lang),
                "phone": c.phone,
            })
        })
        .collect()
}

fn push_contact_slot(
    children: &mut Vec<Component>,
    index: usize,
    contact: Option<&ContactRow>,
    lang: &str,
) {
    let slot = index + 1;
    let label = contact.map(|c| c.label.get(lang)).unwrap_or("");
    let phone = contact.map(|c| c.phone.as_str()).unwrap_or("");

    children.push(
        Text::new()
            .text(json!(format!("i18n:host.contact.slot{slot}")))
            .variant(json!("caption"))
            .into(),
    );
    children.push(
        Field::new()
            .name(json!(format!("contacts.{index}.label")))
            .label(json!("i18n:host.contact.label"))
            .child(
                TextInput::new()
                    .name(json!(format!("contacts.{index}.label")))
                    .value(json!(label)),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(json!(format!("contacts.{index}.phone")))
            .label(json!("i18n:host.contact.phone"))
            .child(
                TextInput::new()
                    .name(json!(format!("contacts.{index}.phone")))
                    .value(json!(phone)),
            )
            .into(),
    );
}
