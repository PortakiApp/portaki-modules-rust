//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Text, TextInput};
use portaki_sdk::sdui::surface::Surface;

use crate::config::{load_config, ContactRow, Localized};

const CONTACT_SLOTS: usize = 6;

#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let lang = Localized::lang_code(&ctx.locale);
    let config = load_config().unwrap_or_default();
    let contacts = config.parse_contacts();

    let submit_args = crate::commands::UpdateConfigArgs {
        contacts: contacts_to_submit(&contacts, &lang),
        contacts_json: String::new(),
        host_visible_phone: config.host_visible_phone.clone(),
    };
    let save_action = Action::command(
        &crate::ids::module_id(),
        crate::ids::UPDATE_CONFIG,
        submit_args,
    );

    let mut form_children: Vec<Component> = vec![Field::new()
        .name("host_visible_phone")
        .label("i18n:host.phone.label")
        .child(
            TextInput::new()
                .name("host_visible_phone")
                .value(config.host_visible_phone)
                .placeholder("i18n:host.phone.placeholder"),
        )
        .into()];

    for index in 0..CONTACT_SLOTS {
        push_contact_slot(&mut form_children, index, contacts.get(index), &lang);
    }

    form_children.push(
        Text::new()
            .text("i18n:host.main.help")
            .variant(TextVariant::Caption)
            .into(),
    );
    form_children.push(
        Button::new()
            .label("i18n:host.save")
            .action(save_action)
            .into(),
    );

    Surface::new(
        Page::new()
            .title("i18n:surface.host.main.title")
            .child(
                Text::new()
                    .text("i18n:surface.host.main.subtitle")
                    .variant(TextVariant::Body),
            )
            .child(Form::new().children(form_children)),
    )
    .with_id(crate::ids::HOST_MAIN)
}

fn contacts_to_submit(contacts: &[ContactRow], lang: &str) -> Vec<crate::commands::ContactInput> {
    contacts
        .iter()
        .map(|c| crate::commands::ContactInput {
            label: c.label.get(lang).to_string(),
            label_fr: String::new(),
            label_en: String::new(),
            phone: c.phone.clone(),
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
            .text(format!("i18n:host.contact.slot{slot}"))
            .variant(TextVariant::Caption)
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("contacts.{index}.label"))
            .label("i18n:host.contact.label")
            .child(
                TextInput::new()
                    .name(format!("contacts.{index}.label"))
                    .value(label),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("contacts.{index}.phone"))
            .label("i18n:host.contact.phone")
            .child(
                TextInput::new()
                    .name(format!("contacts.{index}.phone"))
                    .value(phone),
            )
            .into(),
    );
}
