//! Guest booklet surfaces.

use portaki_sdk::host::module;
use portaki_sdk::prelude::*;
use portaki_sdk::sdui::common::{TempVariant, Tone};
use portaki_sdk::sdui::primitives::{
    Badge, Card, EmptyState, Grid, Stack, Temperature, Text, WeatherIcon,
};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use crate::config::load_config;
use crate::queries::{get_current, get_forecast, GetCurrentArgs, GetForecastArgs};
use crate::weather::{
    convert_temp, has_open_weather, is_uv_high, uv_label_key, WeatherCurrent, WeatherForecast,
};

/// Guest home booklet card with current conditions.
#[portaki_sdk::surface(guest, id = "home.card")]
pub fn render_home_card(ctx: GuestContext) -> Surface {
    match render_home_card_inner(&ctx) {
        Ok(surface) => surface,
        Err(_error) => empty_weather_state(),
    }
}

fn render_home_card_inner(ctx: &GuestContext) -> Result<Surface> {
    if let Some(surface) = empty_state_if_module_not_ready("home.card")? {
        return Ok(surface);
    }
    if !has_open_weather(ctx) {
        return Ok(empty_weather_state());
    }

    let config = load_config()?;
    let current = get_current(
        ctx.clone(),
        GetCurrentArgs {
            lat: None,
            lng: None,
        },
    )?;
    Ok(build_home_card(&current, &config.units))
}

fn build_home_card(current: &WeatherCurrent, units: &crate::entities::WeatherUnits) -> Surface {
    let temp = convert_temp(current.temp_c, *units);
    let unit = units.sdui_unit();
    let description = json!(format!("i18n:{}", current.description_key));

    let mut card_children: Vec<Component> = vec![Component::Stack(
        Stack::new()
            .child(
                WeatherIcon::new()
                    .condition(json!(current.condition.clone()))
                    .size(json!("large")),
            )
            .child(Component::Stack(
                Stack::new()
                    .child(
                        Temperature::new()
                            .value(json!(temp))
                            .unit(json!(unit))
                            .variant(TempVariant::Hero),
                    )
                    .child(Text::new().text(description).variant(json!("caption"))),
            )),
    )];

    if is_uv_high(current.uv_index) {
        let uv_key = current
            .uv_index
            .map(uv_label_key)
            .unwrap_or("weather.uv.high");
        card_children.push(Component::Badge(
            Badge::new()
                .label(json!(format!("i18n:{uv_key}")))
                .tone(Tone::Warning),
        ));
    }

    Surface::new(
        Card::new()
            .tone(Tone::Accent)
            .title(json!("i18n:home.card.title"))
            .children(card_children),
    )
    .with_id("home.card")
}

fn empty_weather_state() -> Surface {
    empty_weather_state_with_id("home.card")
}

fn empty_weather_state_with_id(surface_id: &str) -> Surface {
    Surface::new(
        EmptyState::new()
            .title(json!("i18n:capability.openWeather.fallback"))
            .description(json!("i18n:capability.openWeather.byok.fallback"))
            .icon(json!("cloud-off"))
            .child(
                Text::new()
                    .text(json!("i18n:home.card.unavailable"))
                    .variant(json!("body")),
            ),
    )
    .with_id(surface_id)
}

fn empty_config_state(surface_id: &str) -> Surface {
    Surface::new(
        EmptyState::new()
            .title(json!("i18n:module.status.incomplete.title"))
            .description(json!("i18n:module.status.incomplete.description"))
            .icon(json!("sliders"))
            .child(
                Text::new()
                    .text(json!("i18n:module.status.incomplete.hint"))
                    .variant(json!("body")),
            ),
    )
    .with_id(surface_id)
}

fn empty_inactive_state(surface_id: &str) -> Surface {
    Surface::new(
        EmptyState::new()
            .title(json!("i18n:module.status.inactive.title"))
            .description(json!("i18n:module.status.inactive.description"))
            .icon(json!("cloud-off")),
    )
    .with_id(surface_id)
}

/// Returns an EmptyState when the property-module is not ready to serve guest content.
fn empty_state_if_module_not_ready(surface_id: &str) -> Result<Option<Surface>> {
    let status = module::status()?;
    if !status.workspace_enabled || !status.active {
        return Ok(Some(empty_inactive_state(surface_id)));
    }
    if status.incomplete {
        return Ok(Some(empty_config_state(surface_id)));
    }
    Ok(None)
}

/// Guest explore section with a 5-day forecast grid.
#[portaki_sdk::surface(guest, id = "explore.forecast")]
pub fn render_explore_forecast(ctx: GuestContext) -> Surface {
    match render_explore_forecast_inner(&ctx) {
        Ok(surface) => surface,
        Err(_) => empty_weather_state_with_id("explore.forecast"),
    }
}

fn render_explore_forecast_inner(ctx: &GuestContext) -> Result<Surface> {
    if let Some(surface) = empty_state_if_module_not_ready("explore.forecast")? {
        return Ok(surface);
    }
    if !has_open_weather(ctx) {
        return Ok(empty_weather_state_with_id("explore.forecast"));
    }

    let _config = load_config()?;
    let forecast = get_forecast(
        ctx.clone(),
        GetForecastArgs {
            lat: None,
            lng: None,
            days: Some(5),
        },
    )?;
    Ok(build_forecast_surface(&forecast))
}

fn build_forecast_surface(forecast: &WeatherForecast) -> Surface {
    let unit = forecast.units.sdui_unit();
    let day_columns: Vec<Component> = forecast
        .days
        .iter()
        .map(|day| {
            Component::Stack(
                Stack::new()
                    .child(
                        Text::new()
                            .text(json!(format!("i18n:{}", day.weekday_key)))
                            .variant(json!("caption")),
                    )
                    .child(
                        WeatherIcon::new()
                            .condition(json!(day.condition.clone()))
                            .size(json!("medium")),
                    )
                    .child(
                        Temperature::new()
                            .value(json!(day.display_temp_c))
                            .unit(json!(unit))
                            .variant(TempVariant::Compact),
                    ),
            )
        })
        .collect();

    Surface::new(
        Card::new()
            .title(json!("i18n:explore.forecast.title"))
            .child(
                Grid::new().columns(json!(5)).children(day_columns).child(
                    Text::new()
                        .text(json!("i18n:explore.forecast.hint"))
                        .variant(json!("caption")),
                ),
            ),
    )
    .with_id("explore.forecast")
}
