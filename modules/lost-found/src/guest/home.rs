//! Guest home booklet card — lost/found form and stay report list.

use portaki_sdk::prelude::*;

use portaki_sdk::sdui::primitives::{
    Button, Card, ChoiceList, Field, Form, InfoBanner, ListItem, Stack, Text, TextArea, TextInput,
};
use portaki_sdk::sdui::surface::Surface;

use crate::config::load_config;
use crate::entities::LostFoundReport;
use crate::kind;

pub fn build_home_card(reports: &[LostFoundReport]) -> Surface {
    let config = load_config().unwrap_or_default();
    let mut children: Vec<Component> = Vec::new();

    if let Some(note) = config.host_note_text() {
        children.push(InfoBanner::new().message(note.to_string()).into());
    }

    if reports.is_empty() {
        children.push(
            Text::new()
                .text("i18n:home.card.intro")
                .variant(TextVariant::Body)
                .into(),
        );
    } else {
        children.push(
            Text::new()
                .text("i18n:home.card.thanks")
                .variant(TextVariant::Body)
                .into(),
        );
        children.push(
            Text::new()
                .text("i18n:home.card.yourReports")
                .variant(TextVariant::Caption)
                .into(),
        );
        for report in reports {
            children.push(report_list_item(report).into());
        }
    }

    children.push(build_form().into());

    Surface::new(
        Card::new()
            .icon("search")
            .title("i18n:home.card.title")
            .child(Stack::new().gap(12.0).children(children)),
    )
    .with_id(crate::ids::HOME_CARD)
}

fn report_list_item(report: &LostFoundReport) -> ListItem {
    let subtitle = kind::kind_label_key(report.kind.as_str());
    ListItem::new()
        .title(report.item_description.clone())
        .subtitle(format!("i18n:{subtitle}"))
}

fn build_form() -> Form {
    let submit_action = crate::ids::module_id().command_empty(crate::ids::SUBMIT);

    Form::new()
        .child(
            Field::new()
                .name("kind")
                .label("i18n:form.kind.label")
                .required(true)
                .child(kind_choice_list()),
        )
        .child(
            Field::new()
                .name("itemDescription")
                .label("i18n:form.itemDescription.label")
                .required(true)
                .child(
                    TextInput::new()
                        .name("itemDescription")
                        .placeholder("i18n:form.itemDescription.placeholder"),
                ),
        )
        .child(
            Field::new()
                .name("contactHint")
                .label("i18n:form.contactHint.label")
                .child(
                    TextInput::new()
                        .name("contactHint")
                        .placeholder("i18n:form.contactHint.placeholder"),
                ),
        )
        .child(
            Field::new()
                .name("details")
                .label("i18n:form.details.label")
                .child(
                    TextArea::new()
                        .name("details")
                        .placeholder("i18n:form.details.placeholder"),
                ),
        )
        .child(
            Button::new()
                .label("i18n:form.submit")
                .action(submit_action),
        )
}

fn kind_choice_list() -> ChoiceList {
    ChoiceList::new()
        .name("kind")
        .layout(ChoiceListLayout::Compact)
        .choices(vec![
            ChoiceOption::new("lost", "i18n:form.kind.lost").icon("search-x"),
            ChoiceOption::new("found", "i18n:form.kind.found").icon("package-search"),
        ])
}
