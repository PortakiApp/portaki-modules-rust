//! Guest home booklet card — report form and stay report list.

use portaki_sdk::prelude::*;

use portaki_sdk::sdui::primitives::{
    Button, Card, ChoiceList, Field, Form, ListItem, Stack, Text, TextArea, TextInput,
};
use portaki_sdk::sdui::surface::Surface;

use crate::category;
use crate::entities::IssueReport;

pub fn build_home_card(reports: &[IssueReport]) -> Surface {
    let mut children: Vec<Component> = Vec::new();

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
            .icon("triangle-alert")
            .title("i18n:home.card.title")
            .child(Stack::new().gap(12.0).children(children)),
    )
    .with_id(crate::ids::HOME_CARD)
}

fn report_list_item(report: &IssueReport) -> ListItem {
    let subtitle = category::category_label_key(report.category.as_str());
    ListItem::new()
        .title(report.summary.clone())
        .subtitle(format!("i18n:{subtitle}"))
}

fn build_form() -> Form {
    let submit_action = crate::ids::module_id().command_empty(crate::ids::SUBMIT);

    Form::new()
        .child(
            Field::new()
                .name("category")
                .label("i18n:form.category.label")
                .required(true)
                .child(category_choice_list()),
        )
        .child(
            Field::new()
                .name("summary")
                .label("i18n:form.summary.label")
                .required(true)
                .child(
                    TextInput::new()
                        .name("summary")
                        .placeholder("i18n:form.summary.placeholder"),
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

fn category_choice_list() -> ChoiceList {
    ChoiceList::new()
        .name("category")
        .layout(ChoiceListLayout::Compact)
        .choices(vec![
            ChoiceOption::new("appliance", "i18n:form.category.appliance").icon("plug"),
            ChoiceOption::new("cleanliness", "i18n:form.category.cleanliness").icon("sparkles"),
            ChoiceOption::new("noise", "i18n:form.category.noise").icon("volume-2"),
            ChoiceOption::new("access", "i18n:form.category.access").icon("key"),
            ChoiceOption::new("other", "i18n:form.category.other").icon("message-circle"),
        ])
}
