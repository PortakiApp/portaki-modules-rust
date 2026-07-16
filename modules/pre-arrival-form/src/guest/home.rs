//! Guest home booklet card — pre-arrival form or thank-you state.

use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{
    Button, Card, Field, Form, Text, TextArea, TextInput, TimePicker,
};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

pub fn build_form_card() -> Surface {
    let submit_action =
        serde_json::to_value(Action::command("pre-arrival-form", "submit", json!({})))
            .unwrap_or(json!({}));

    Surface::new(
        Card::new()
            .icon(json!("clipboard-list"))
            .title(json!("i18n:home.card.title"))
            .child(
                Text::new()
                    .text(json!("i18n:home.card.intro"))
                    .variant(json!("body")),
            )
            .child(
                Form::new()
                    .child(
                        Field::new()
                            .name(json!("arrivalTimeEstimated"))
                            .label(json!("i18n:form.arrival.label"))
                            .required(json!(true))
                            .child(TimePicker::new().name(json!("arrivalTimeEstimated"))),
                    )
                    .child(
                        Field::new()
                            .name(json!("guestOccasion"))
                            .label(json!("i18n:form.occasion.label"))
                            .child(
                                TextInput::new()
                                    .name(json!("guestOccasion"))
                                    .placeholder(json!("i18n:form.occasion.placeholder")),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("guestAllergies"))
                            .label(json!("i18n:form.allergies.label"))
                            .child(
                                TextInput::new()
                                    .name(json!("guestAllergies"))
                                    .placeholder(json!("i18n:form.allergies.placeholder")),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("messageToHost"))
                            .label(json!("i18n:form.message.label"))
                            .child(
                                TextArea::new()
                                    .name(json!("messageToHost"))
                                    .placeholder(json!("i18n:form.message.placeholder")),
                            ),
                    )
                    .child(
                        Button::new()
                            .label(json!("i18n:form.submit"))
                            .action(submit_action),
                    ),
            ),
    )
    .with_id("home.card")
}

pub fn build_completed_card() -> Surface {
    Surface::new(
        Card::new()
            .icon(json!("clipboard-list"))
            .title(json!("i18n:home.card.title"))
            .child(
                Text::new()
                    .text(json!("i18n:home.card.thanks"))
                    .variant(json!("body")),
            ),
    )
    .with_id("home.card")
}
