//! Guest home booklet card — pre-arrival form or thank-you state.

use portaki_sdk::prelude::*;

use portaki_sdk::sdui::primitives::{
    Button, Card, Field, Form, Text, TextArea, TextInput, TimePicker,
};
use portaki_sdk::sdui::surface::Surface;

pub fn build_form_card() -> Surface {
    let submit_action = crate::ids::module_id().command_empty(crate::ids::SUBMIT);

    Surface::new(
        Card::new()
            .icon("clipboard-list")
            .title("i18n:home.card.title")
            .child(
                Text::new()
                    .text("i18n:home.card.intro")
                    .variant(TextVariant::Body),
            )
            .child(
                Form::new()
                    .child(
                        Field::new()
                            .name("arrivalTimeEstimated")
                            .label("i18n:form.arrival.label")
                            .required(true)
                            .child(TimePicker::new().name("arrivalTimeEstimated")),
                    )
                    .child(
                        Field::new()
                            .name("guestOccasion")
                            .label("i18n:form.occasion.label")
                            .child(
                                TextInput::new()
                                    .name("guestOccasion")
                                    .placeholder("i18n:form.occasion.placeholder"),
                            ),
                    )
                    .child(
                        Field::new()
                            .name("guestAllergies")
                            .label("i18n:form.allergies.label")
                            .child(
                                TextInput::new()
                                    .name("guestAllergies")
                                    .placeholder("i18n:form.allergies.placeholder"),
                            ),
                    )
                    .child(
                        Field::new()
                            .name("messageToHost")
                            .label("i18n:form.message.label")
                            .child(
                                TextArea::new()
                                    .name("messageToHost")
                                    .placeholder("i18n:form.message.placeholder"),
                            ),
                    )
                    .child(
                        Button::new()
                            .label("i18n:form.submit")
                            .action(submit_action),
                    ),
            ),
    )
    .with_id(crate::ids::HOME_CARD)
}

pub fn build_completed_card() -> Surface {
    Surface::new(
        Card::new()
            .icon("clipboard-list")
            .title("i18n:home.card.title")
            .child(
                Text::new()
                    .text("i18n:home.card.thanks")
                    .variant(TextVariant::Body),
            ),
    )
    .with_id(crate::ids::HOME_CARD)
}
