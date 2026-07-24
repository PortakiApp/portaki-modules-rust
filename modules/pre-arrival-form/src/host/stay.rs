//! Stay-scoped host surface — design stay detail « Formulaire de pré-arrivée ».

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::common::Tone;
use portaki_sdk::sdui::primitives::{Card, ListItem, Page, Pill, Stack, Text};
use portaki_sdk::sdui::surface::Surface;
use uuid::Uuid;

use crate::entities::PreArrivalResponse;
use crate::storage;

/// Stay detail embed — read-only pre-arrival responses for `input.stayId`.
#[portaki_sdk::surface(host, id = "stay")]
pub fn render_host_stay(ctx: HostContext) -> Surface {
    let stay_id = ctx
        .input_str("stayId")
        .and_then(|raw| Uuid::parse_str(raw).ok());

    let body = match stay_id {
        None => missing_stay_card(),
        Some(stay_id) => match storage::find_by_stay(stay_id).ok().flatten() {
            Some(row) => completed_card(&row),
            None => pending_card(),
        },
    };

    Surface::new(Page::new().child(body)).with_id(crate::ids::HOST_STAY)
}

fn missing_stay_card() -> Component {
    Component::Card(
        Card::new()
            .icon("clipboard-list")
            .title("i18n:surface.host.stay.title")
            .child(
                Text::new()
                    .text("i18n:host.stay.missingStay")
                    .variant(TextVariant::Caption),
            ),
    )
}

fn pending_card() -> Component {
    let status = Pill::new()
        .label("i18n:host.stay.status.pending")
        .tone(Tone::Neutral);

    Component::Card(
        Card::new()
            .icon("clipboard-list")
            .title("i18n:surface.host.stay.title")
            .child(Stack::new().gap(12.0).children(vec![
                        status.into(),
                        Text::new()
                            .text("i18n:host.stay.pending")
                            .variant(TextVariant::Caption)
                            .into(),
                    ])),
    )
}

fn completed_card(row: &PreArrivalResponse) -> Component {
    let status = Pill::new()
        .label("i18n:host.stay.status.done")
        .tone(Tone::Success);

    let arrival = display_or_dash(row.arrival_time.as_deref());
    let occasion = display_or_dash(row.occasion.as_deref());
    let allergies = allergies_display(row.allergies.as_deref());

    let mut rows: Vec<Component> = vec![
        status.into(),
        detail_row("🕐", "i18n:form.arrival.label", arrival),
        detail_row("✨", "i18n:form.occasion.label", occasion),
        detail_row("⚠️", "i18n:form.allergies.label", allergies),
    ];

    if let Some(message) = row
        .guest_message
        .as_ref()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    {
        rows.push(detail_row(
            "💬",
            "i18n:form.message.label",
            message.to_string(),
        ));
    }

    Component::Card(
        Card::new()
            .icon("clipboard-list")
            .title("i18n:surface.host.stay.title")
            .child(Stack::new().gap(4.0).children(rows)),
    )
}

fn detail_row(leading: &str, label_i18n: &str, value: String) -> Component {
    Component::ListItem(
        ListItem::new()
            .title(label_i18n)
            .subtitle(value)
            .leading(leading)
            .chevron(false),
    )
}

fn display_or_dash(value: Option<&str>) -> String {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("—")
        .to_string()
}

fn allergies_display(value: Option<&str>) -> String {
    match value.map(str::trim).filter(|value| !value.is_empty()) {
        Some(allergies) => allergies.to_string(),
        None => "i18n:host.stay.allergies.none".to_string(),
    }
}
