//! Host create-found form (TipTap description, stay picker, no status).

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::common::Tone;
use portaki_sdk::sdui::primitives::{Button, Field, Form, ListItem, RichTextEditor, Stack, Text};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::commands::SubmitFoundArgs;

/// Stay option passed by the host shell via `HostContext.input.stays`.
#[derive(Debug, Clone)]
struct StayOption {
    id: Uuid,
    label: String,
}

/// Builds the declare-found form.
///
/// - `input.stayId` — fixed stay (stay detail); picker hidden.
/// - `input.stays` + `input.stayIds` — multi-select via surface input emits.
/// - Status is never shown; [`submit_found`](crate::submit_found) always uses
///   `to_collect`.
pub(crate) fn build_create_found_form(ctx: &HostContext) -> Component {
    let fixed_stay = ctx
        .input_str("stayId")
        .and_then(|raw| Uuid::parse_str(raw).ok());
    let catalog = stay_catalog(ctx);
    let mut selected = selected_stay_ids(ctx);

    if let Some(stay_id) = fixed_stay {
        selected = vec![stay_id];
    }

    let mut children: Vec<Component> = vec![
        Text::new()
            .text("i18n:host.create.title")
            .variant(TextVariant::Title)
            .into(),
        Text::new()
            .text("i18n:host.create.help")
            .variant(TextVariant::Caption)
            .into(),
    ];

    if fixed_stay.is_none() {
        if catalog.is_empty() {
            children.push(
                Text::new()
                    .text("i18n:host.create.noStays")
                    .variant(TextVariant::Caption)
                    .into(),
            );
        } else {
            children.push(
                Text::new()
                    .text("i18n:host.create.stays.label")
                    .variant(TextVariant::Caption)
                    .into(),
            );
            let items: Vec<Component> = catalog
                .iter()
                .map(|stay| {
                    let checked = selected.contains(&stay.id);
                    let mut item = ListItem::new()
                        .title(stay.label.clone())
                        .chevron(false)
                        .action(emit_toggle_stay(stay.id, &selected));
                    if checked {
                        item = item.tone(Tone::Primary);
                    }
                    Component::ListItem(item)
                })
                .collect();
            children.push(Component::List(
                portaki_sdk::sdui::primitives::List::new().children(items),
            ));
        }
    }

    let submit_action = crate::ids::module_id().command(
        crate::ids::SUBMIT_FOUND,
        SubmitFoundArgs {
            stay_ids: selected.clone(),
            stay_id: None,
            description: String::new(),
            status: None,
        },
    );

    let can_submit = !selected.is_empty();
    let mut submit = Button::new()
        .label("i18n:host.create.submit")
        .action(submit_action);
    if can_submit {
        submit = submit.tone(Tone::Primary);
    }

    children.push(
        Form::new()
            .child(
                Field::new()
                    .name("description")
                    .label("i18n:host.create.description.label")
                    .required(true)
                    .child(RichTextEditor::new().name("description")),
            )
            .child(submit)
            .into(),
    );

    Component::Stack(Stack::new().gap(12.0).children(children))
}

fn stay_catalog(ctx: &HostContext) -> Vec<StayOption> {
    let Some(raw) = ctx.input.get("stays") else {
        return Vec::new();
    };
    let Some(arr) = raw.as_array() else {
        return Vec::new();
    };

    arr.iter()
        .filter_map(|entry| {
            let id = entry
                .get("id")
                .and_then(Value::as_str)
                .and_then(|s| Uuid::parse_str(s).ok())?;
            let label = entry
                .get("label")
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim()
                .to_string();
            let label = if label.is_empty() {
                id.to_string()
            } else {
                label
            };
            Some(StayOption { id, label })
        })
        .collect()
}

fn selected_stay_ids(ctx: &HostContext) -> Vec<Uuid> {
    if let Some(id) = ctx
        .input_str("stayId")
        .and_then(|raw| Uuid::parse_str(raw).ok())
    {
        return vec![id];
    }

    match ctx.input.get("stayIds") {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(|v| v.as_str())
            .filter_map(|s| Uuid::parse_str(s).ok())
            .collect(),
        Some(Value::String(raw)) => raw
            .split(',')
            .filter_map(|s| Uuid::parse_str(s.trim()).ok())
            .collect(),
        _ => Vec::new(),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StayIdsInput {
    stay_ids: Vec<String>,
}

fn emit_toggle_stay(stay_id: Uuid, selected: &[Uuid]) -> Action {
    let mut next: Vec<Uuid> = selected.to_vec();
    if let Some(pos) = next.iter().position(|id| *id == stay_id) {
        next.remove(pos);
    } else {
        next.push(stay_id);
    }
    next.sort_unstable();
    next.dedup();

    let payload = StayIdsInput {
        stay_ids: next.iter().map(Uuid::to_string).collect(),
    };
    Action::emit(contracts::shell::SURFACE_INPUT, Some(json_value(payload)))
}
