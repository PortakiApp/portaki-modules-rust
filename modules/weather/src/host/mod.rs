//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Select, Text};
use portaki_sdk::sdui::surface::Surface;

use crate::config::load_config;
use crate::entities::WeatherUnits;

/// Host configuration page (units + refresh cadence).
#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let _ = ctx;
    let config = load_config().unwrap_or_default();

    let units_value = match config.units {
        WeatherUnits::Celsius => "celsius",
        WeatherUnits::Fahrenheit => "fahrenheit",
    };

    let submit_args = crate::commands::UpdateConfigArgs {
        units: units_value.to_string(),
        refresh_interval: config.refresh_interval.clone(),
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
                            .name("units")
                            .label("i18n:host.units.label")
                            .child(
                                Select::new()
                                    .name("units")
                                    .options(vec![
                                        ChoiceOption::new("celsius", "°C"),
                                        ChoiceOption::new("fahrenheit", "°F"),
                                    ])
                                    .value(units_value),
                            ),
                    )
                    .child(
                        Field::new()
                            .name("refresh_interval")
                            .label("i18n:host.refresh.label")
                            .child(
                                Select::new()
                                    .name("refresh_interval")
                                    .options(vec![
                                        ChoiceOption::new("1h", "i18n:host.hourly"),
                                        ChoiceOption::new("3h", "i18n:host.3hours"),
                                        ChoiceOption::new("6h", "i18n:host.6hours"),
                                    ])
                                    .value(config.refresh_interval),
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
