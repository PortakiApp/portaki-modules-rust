//! Shared status Select options for host report rows.

use portaki_sdk::prelude::*;

pub(crate) fn status_choice_options() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("to_collect", "i18n:status.to_collect"),
        ChoiceOption::new("sent", "i18n:status.sent"),
        ChoiceOption::new("returned", "i18n:status.returned"),
    ]
}
