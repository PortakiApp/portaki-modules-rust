//! Host dashboard surface — module info and recent reports.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{List, ListItem, Page, Text};
use portaki_sdk::sdui::surface::Surface;

use crate::category;
use crate::storage;

/// Host main — explains guest reporting and lists recent property reports.
#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let _ = ctx;
    let reports = storage::list_recent().unwrap_or_default();

    let mut children: Vec<Component> = vec![
        Text::new()
            .text("i18n:surface.host.main.subtitle")
            .variant(TextVariant::Body)
            .into(),
        Text::new()
            .text("i18n:host.main.help")
            .variant(TextVariant::Caption)
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
                let label_key = category::category_label_key(report.category.as_str());
                Component::ListItem(
                    ListItem::new()
                        .title(report.summary.clone())
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
