//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Select, Text};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

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

    let submit_args = json!({
        "units": units_value,
        "refresh_interval": config.refresh_interval,
    });
    let save_action = serde_json::to_value(Action::command("weather", "updateConfig", submit_args))
        .unwrap_or(json!({}));

    Surface::new(
        Page::new()
            .title(json!("i18n:surface.host.main.title"))
            .child(
                Text::new()
                    .text(json!("i18n:surface.host.main.subtitle"))
                    .variant(json!("body")),
            )
            .child(
                Form::new()
                    .child(
                        Field::new()
                            .name(json!("units"))
                            .label(json!("i18n:host.units.label"))
                            .child(
                                Select::new()
                                    .name(json!("units"))
                                    .options(json!([
                                        {"value": "celsius", "label": "°C"},
                                        {"value": "fahrenheit", "label": "°F"}
                                    ]))
                                    .value(json!(units_value)),
                            ),
                    )
                    .child(
                        Field::new()
                            .name(json!("refresh_interval"))
                            .label(json!("i18n:host.refresh.label"))
                            .child(
                                Select::new()
                                    .name(json!("refresh_interval"))
                                    .options(json!([
                                        {"value": "1h", "label": "i18n:host.hourly"},
                                        {"value": "3h", "label": "i18n:host.3hours"},
                                        {"value": "6h", "label": "i18n:host.6hours"}
                                    ]))
                                    .value(json!(config.refresh_interval)),
                            ),
                    )
                    .child(
                        Text::new()
                            .text(json!("i18n:host.main.help"))
                            .variant(json!("caption")),
                    )
                    .child(
                        Button::new()
                            .label(json!("i18n:host.save"))
                            .action(save_action),
                    ),
            ),
    )
    .with_id("main")
}
