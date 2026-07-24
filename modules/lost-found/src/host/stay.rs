//! Stay-scoped host surface — create-found + reports for one stay.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{Button, Field, Form, List, Page, Select, Text};
use portaki_sdk::sdui::surface::Surface;
use uuid::Uuid;

use crate::commands::UpdateStatusArgs;
use crate::description;
use crate::storage;

use super::{build_create_found_form, status_choice_options};

/// Stay detail embed — declare found + list/update status for `input.stayId`.
#[portaki_sdk::surface(host, id = "stay")]
pub fn render_host_stay(ctx: HostContext) -> Surface {
    let stay_id = ctx
        .input_str("stayId")
        .and_then(|raw| Uuid::parse_str(raw).ok());

    let mut children: Vec<Component> = vec![build_create_found_form(&ctx)];

    match stay_id {
        None => {
            children.push(
                Text::new()
                    .text("i18n:host.stay.missingStay")
                    .variant(TextVariant::Caption)
                    .into(),
            );
        }
        Some(stay_id) => {
            let reports = storage::list_by_stay(stay_id).unwrap_or_default();
            if reports.is_empty() {
                children.push(
                    Text::new()
                        .text("i18n:host.stay.empty")
                        .variant(TextVariant::Caption)
                        .into(),
                );
            } else {
                children.push(
                    Text::new()
                        .text("i18n:host.stay.listTitle")
                        .variant(TextVariant::Title)
                        .into(),
                );
                let items: Vec<Component> = reports
                    .iter()
                    .map(|report| {
                        let title = description::to_plain_text(&report.item_description);
                        let title = if title.is_empty() {
                            report.item_description.clone()
                        } else {
                            title
                        };
                        let kind_label = if report.kind == "found" {
                            "i18n:host.stay.kind.found"
                        } else {
                            "i18n:host.stay.kind.lost"
                        };
                        let update_action = crate::ids::module_id().command(
                            crate::ids::UPDATE_STATUS,
                            UpdateStatusArgs {
                                report_id: report.id,
                                status: report.status.clone(),
                            },
                        );
                        Component::Form(
                            Form::new()
                                .child(Text::new().text(title).variant(TextVariant::Body))
                                .child(Text::new().text(kind_label).variant(TextVariant::Caption))
                                .child(
                                    Field::new()
                                        .name("status")
                                        .label("i18n:host.main.status.label")
                                        .child(
                                            Select::new()
                                                .name("status")
                                                .options(status_choice_options())
                                                .value(report.status.as_str()),
                                        ),
                                )
                                .child(
                                    Button::new()
                                        .label("i18n:host.main.updateStatus")
                                        .action(update_action),
                                ),
                        )
                    })
                    .collect();
                children.push(Component::List(List::new().children(items)));
            }
        }
    }

    Surface::new(
        Page::new()
            .title("i18n:surface.host.stay.title")
            .children(children),
    )
    .with_id(crate::ids::HOST_STAY)
}
