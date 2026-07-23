//! Host dashboard surface — guest tip, recent reports.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{Button, Field, Form, List, ListItem, Page, Text, TextArea};
use portaki_sdk::sdui::surface::Surface;

use crate::config::load_config;
use crate::kind;
use crate::storage;

/// Host main — optional guest tip, config form, recent property reports.
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
        Text::new()
            .text("i18n:surface.host.main.subtitle")
            .variant(TextVariant::Body)
            .into(),
        Text::new()
            .text("i18n:host.main.help")
            .variant(TextVariant::Caption)
            .into(),
        Form::new()
            .child(
                Field::new()
                    .name("host_note")
                    .label("i18n:host.hostNote.label")
                    .child(
                        TextArea::new()
                            .name("host_note")
                            .value(host_note)
                            .placeholder("i18n:host.hostNote.placeholder"),
                    ),
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
                let label_key = kind::kind_label_key(report.kind.as_str());
                Component::ListItem(
                    ListItem::new()
                        .title(report.item_description.clone())
                        .subtitle(format!("i18n:{label_key}")),
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
