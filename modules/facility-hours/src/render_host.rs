//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Text, TextArea, TextInput};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use crate::config::{load_config, FacilityRow, Localized};

const FACILITY_SLOTS: usize = 6;

#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let lang = Localized::lang_code(&ctx.locale);
    let config = load_config().unwrap_or_default();
    let facilities = config.parse_facilities();
    let general_note = config.general_note.get(&lang).to_string();

    let submit_args = json!({
        "facilities": facilities_to_submit(&facilities, &lang),
        "general_note": general_note,
    });
    let save_action = serde_json::to_value(Action::command(
        "facility-hours",
        "updateConfig",
        submit_args,
    ))
    .unwrap_or(json!({}));

    let mut form_children: Vec<Component> = Vec::new();
    for index in 0..FACILITY_SLOTS {
        push_facility_slot(&mut form_children, index, facilities.get(index), &lang);
    }
    form_children.push(
        Field::new()
            .name(json!("general_note"))
            .label(json!("i18n:host.note.label"))
            .child(
                TextArea::new()
                    .name(json!("general_note"))
                    .value(json!(general_note))
                    .placeholder(json!("i18n:host.note.placeholder")),
            )
            .into(),
    );
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

fn facilities_to_submit(facilities: &[FacilityRow], lang: &str) -> Vec<serde_json::Value> {
    facilities
        .iter()
        .map(|f| {
            json!({
                "name": f.title.get(lang),
                "hours": f.hours.clone().unwrap_or_default(),
            })
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
            .text(json!(format!("i18n:host.facility.slot{slot}")))
            .variant(json!("caption"))
            .into(),
    );
    children.push(
        Field::new()
            .name(json!(format!("facilities.{index}.name")))
            .label(json!("i18n:host.facility.name"))
            .child(
                TextInput::new()
                    .name(json!(format!("facilities.{index}.name")))
                    .value(json!(name)),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(json!(format!("facilities.{index}.hours")))
            .label(json!("i18n:host.facility.hours"))
            .child(
                TextInput::new()
                    .name(json!(format!("facilities.{index}.hours")))
                    .value(json!(hours))
                    .placeholder(json!("i18n:host.facility.hours.placeholder")),
            )
            .into(),
    );
}
