//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{
    Button, Field, Form, Page, Select, Text, TextArea, TextInput, Toggle,
};
use portaki_sdk::sdui::surface::Surface;

use crate::config::{load_config, Localized};

#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let lang = Localized::lang_code(&ctx.locale);
    let config = load_config().unwrap_or_default();
    let thank_you_message = config.thank_you_message.get(&lang).to_string();

    let submit_args = crate::commands::UpdateConfigArgs {
        review_channel: config.review_channel.as_str().to_string(),
        show_qr_code: config.show_qr_code,
        airbnb_review_url: config.airbnb_review_url.clone(),
        thank_you_message: thank_you_message.clone(),
    };
    let save_action = crate::ids::module_id().command(crate::ids::UPDATE_CONFIG, submit_args);

    Surface::new(
        Page::new()
            .title("i18n:surface.host.main.title")
            .child(
                Text::new()
                    .text("i18n:surface.host.main.subtitle")
                    .variant(TextVariant::Body),
            )
            .child(
                Form::new()
                    .child(
                        Field::new()
                            .name("review_channel")
                            .label("i18n:host.channel.label")
                            .child(
                                Select::new()
                                    .name("review_channel")
                                    .options(vec![
                                        ChoiceOption::new("airbnb", "i18n:host.channel.airbnb"),
                                        ChoiceOption::new("portaki", "i18n:host.channel.portaki"),
                                        ChoiceOption::new("both", "i18n:host.channel.both"),
                                    ])
                                    .value(config.review_channel.as_str()),
                            ),
                    )
                    .child(
                        Field::new()
                            .name("show_qr_code")
                            .label("i18n:host.qr.label")
                            .child(
                                Toggle::new()
                                    .name("show_qr_code")
                                    .checked(config.show_qr_code),
                            ),
                    )
                    .child(
                        Field::new()
                            .name("airbnb_review_url")
                            .label("i18n:host.airbnb.label")
                            .child(
                                TextInput::new()
                                    .name("airbnb_review_url")
                                    .value(config.airbnb_review_url)
                                    .placeholder("i18n:host.airbnb.placeholder"),
                            ),
                    )
                    .child(
                        Field::new()
                            .name("thank_you_message")
                            .label("i18n:host.thanks.label")
                            .child(
                                TextArea::new()
                                    .name("thank_you_message")
                                    .value(thank_you_message)
                                    .placeholder("i18n:host.thanks.placeholder"),
                            ),
                    )
                    .child(
                        Text::new()
                            .text("i18n:host.main.help")
                            .variant(TextVariant::Caption),
                    )
                    .child(Button::new().label("i18n:host.save").action(save_action)),
            ),
    )
    .with_id(crate::ids::HOST_MAIN)
}
