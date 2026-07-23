//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Text, TextArea, TextInput};
use portaki_sdk::sdui::surface::Surface;

use crate::config::{load_config, FacilityRow, Localized};

const FACILITY_SLOTS: usize = 6;

#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let lang = Localized::lang_code(&ctx.locale);
    let config = load_config().unwrap_or_default();
    let facilities = config.parse_facilities();
    let general_note = config.general_note.get(&lang).to_string();

    let submit_args = crate::commands::UpdateConfigArgs {
        facilities: facilities_to_submit(&facilities, &lang),
        facilities_json: String::new(),
        general_note: general_note.clone(),
    };
    let save_action = crate::ids::module_id().command(crate::ids::UPDATE_CONFIG, submit_args);

    let mut form_children: Vec<Component> = Vec::new();
    for index in 0..FACILITY_SLOTS {
        push_facility_slot(&mut form_children, index, facilities.get(index), &lang);
    }
    form_children.push(
        Field::new()
            .name("general_note")
            .label("i18n:host.note.label")
            .child(
                TextArea::new()
                    .name("general_note")
                    .value(general_note)
                    .placeholder("i18n:host.note.placeholder"),
            )
            .into(),
    );
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

fn facilities_to_submit(
    facilities: &[FacilityRow],
    lang: &str,
) -> Vec<crate::commands::FacilityInput> {
    facilities
        .iter()
        .map(|f| crate::commands::FacilityInput {
            name: f.title.get(lang).to_string(),
            name_fr: String::new(),
            name_en: String::new(),
            hours: f.hours.clone().unwrap_or_default(),
        })
        .collect()
}

fn push_facility_slot(
    children: &mut Vec<Component>,
    index: usize,
    facility: Option<&FacilityRow>,
    lang: &str,
) {
    let slot = index + 1;
    let name = facility.map(|f| f.title.get(lang)).unwrap_or("");
    let hours = facility.and_then(|f| f.hours.as_deref()).unwrap_or("");

    children.push(
        Text::new()
            .text(format!("i18n:host.facility.slot{slot}"))
            .variant(TextVariant::Caption)
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("facilities.{index}.name"))
            .label("i18n:host.facility.name")
            .child(
                TextInput::new()
                    .name(format!("facilities.{index}.name"))
                    .value(name),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("facilities.{index}.hours"))
            .label("i18n:host.facility.hours")
            .child(
                TextInput::new()
                    .name(format!("facilities.{index}.hours"))
                    .value(hours)
                    .placeholder("i18n:host.facility.hours.placeholder"),
            )
            .into(),
    );
}
