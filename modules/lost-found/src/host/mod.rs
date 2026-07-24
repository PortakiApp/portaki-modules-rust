//! Host dashboard surface — guest tip, recent reports with status edit (create UI is React).

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{
    Button, Field, Form, InfoBanner, List, Page, RichTextEditor, Select, Text,
};
use portaki_sdk::sdui::surface::Surface;

use crate::commands::UpdateStatusArgs;
use crate::config::load_config;
use crate::description;
use crate::storage;

/// Host main — banner, optional guest tip (TipTap), recent property reports + status.
#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let _ = ctx;
    let config = load_config().unwrap_or_default();
    let host_note = config.host_note.clone().unwrap_or_default();
    let reports = storage::list_recent().unwrap_or_default();

    let save_action = crate::ids::module_id().command(
        crate::ids::UPDATE_CONFIG,
        crate::commands::UpdateConfigArgs {
            host_note: host_note.clone(),
        },
    );

    let mut children: Vec<Component> = vec![
        InfoBanner::new().message("i18n:host.main.banner").into(),
        Text::new()
            .text("i18n:host.main.help")
            .variant(TextVariant::Caption)
            .into(),
        Form::new()
            .child(
                Field::new()
                    .name("host_note")
                    .label("i18n:host.hostNote.label")
                    .child(RichTextEditor::new().name("host_note").value(host_note)),
            )
            .child(Button::new().label("i18n:host.save").action(save_action))
            .into(),
    ];

    if reports.is_empty() {
        children.push(
            Text::new()
                .text("i18n:host.main.emptyRecent")
                .variant(TextVariant::Caption)
                .into(),
        );
    } else {
        children.push(
            Text::new()
                .text("i18n:host.main.recentTitle")
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

    Surface::new(
        Page::new()
            .title("i18n:surface.host.main.title")
            .children(children),
    )
    .with_id(crate::ids::HOST_MAIN)
}

fn status_choice_options() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("to_collect", "i18n:status.to_collect"),
        ChoiceOption::new("sent", "i18n:status.sent"),
        ChoiceOption::new("returned", "i18n:status.returned"),
    ]
}
