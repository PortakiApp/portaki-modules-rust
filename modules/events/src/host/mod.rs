//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Text, TextArea, TextInput};
use portaki_sdk::sdui::surface::Surface;

use crate::config::{load_config, EventRow, Localized};

const EVENT_SLOTS: usize = 6;

#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let lang = Localized::lang_code(&ctx.locale);
    let config = load_config().unwrap_or_default();
    let events = config.parse_events();
    let disclaimer = config.disclaimer.get(&lang).to_string();

    let submit_args = crate::commands::UpdateConfigArgs {
        events: events_to_submit(&events, &lang),
        disclaimer: disclaimer.clone(),
    };
    let save_action = crate::ids::module_id().command(crate::ids::UPDATE_CONFIG, submit_args);

    let mut form_children: Vec<Component> = Vec::new();
    for index in 0..EVENT_SLOTS {
        push_event_slot(&mut form_children, index, events.get(index), &lang);
    }
    form_children.push(
        Field::new()
            .name("disclaimer")
            .label("i18n:host.disclaimer.label")
            .child(
                TextArea::new()
                    .name("disclaimer")
                    .value(disclaimer)
                    .placeholder("i18n:host.disclaimer.placeholder"),
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

fn events_to_submit(events: &[EventRow], lang: &str) -> Vec<crate::commands::EventInput> {
    events
        .iter()
        .map(|event| crate::commands::EventInput {
            title: event.title.get(lang).to_string(),
            place: event.place.get(lang).to_string(),
            starts_at: event.starts_at.clone(),
            url: event.url.clone().unwrap_or_default(),
            lat: event.lat.map(|v| v.to_string()).unwrap_or_default(),
            lng: event.lng.map(|v| v.to_string()).unwrap_or_default(),
        })
        .collect()
}

fn push_event_slot(
    children: &mut Vec<Component>,
    index: usize,
    event: Option<&EventRow>,
    lang: &str,
) {
    let slot = index + 1;
    let title = event.map(|e| e.title.get(lang)).unwrap_or("");
    let place = event.map(|e| e.place.get(lang)).unwrap_or("");
    let starts_at = event.map(|e| e.starts_at.as_str()).unwrap_or("");
    let url = event.and_then(|e| e.url.as_deref()).unwrap_or("");
    let lat = event
        .and_then(|e| e.lat)
        .map(|v| v.to_string())
        .unwrap_or_default();
    let lng = event
        .and_then(|e| e.lng)
        .map(|v| v.to_string())
        .unwrap_or_default();

    children.push(
        Text::new()
            .text(format!("i18n:host.event.slot{slot}"))
            .variant(TextVariant::Caption)
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("events.{index}.title"))
            .label("i18n:host.event.title")
            .child(
                TextInput::new()
                    .name(format!("events.{index}.title"))
                    .value(title),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("events.{index}.place"))
            .label("i18n:host.event.place")
            .child(
                TextInput::new()
                    .name(format!("events.{index}.place"))
                    .value(place),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("events.{index}.starts_at"))
            .label("i18n:host.event.startsAt")
            .child(
                TextInput::new()
                    .name(format!("events.{index}.starts_at"))
                    .value(starts_at)
                    .placeholder("i18n:host.event.startsAt.placeholder"),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("events.{index}.url"))
            .label("i18n:host.event.url")
            .child(
                TextInput::new()
                    .name(format!("events.{index}.url"))
                    .value(url),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("events.{index}.lat"))
            .label("i18n:host.event.lat")
            .child(
                TextInput::new()
                    .name(format!("events.{index}.lat"))
                    .value(lat),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("events.{index}.lng"))
            .label("i18n:host.event.lng")
            .child(
                TextInput::new()
                    .name(format!("events.{index}.lng"))
                    .value(lng),
            )
            .into(),
    );
}
