//! Shared weather body pieces (hero, UV badge, 5-day strip).

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::common::{Emphasis, Tone};
use portaki_sdk::sdui::primitives::{Badge, Divider, Grid, Icon, Stack, Text};
use serde_json::json;

use crate::entities::WeatherUnits;
use crate::weather::{
    convert_temp, format_day_strip_label, format_temp_label, icon_name_for_condition, is_uv_high,
    tone_for_temp_c, uv_label_key, ForecastDayView, WeatherCurrent, WeatherForecast,
};

/// Glance body for the home card: hero + optional UV + forecast strip.
pub fn build_weather_glance(
    current: &WeatherCurrent,
    forecast: &WeatherForecast,
    units: &WeatherUnits,
    city: Option<&str>,
    locale: &str,
) -> Vec<Component> {
    let mut children = vec![build_current_hero(current, units, city)];
    if let Some(badge) = uv_warning_badge(current) {
        children.push(badge);
    }
    children.push(Component::Divider(Divider::new()));
    children.push(build_forecast_strip(forecast, units, locale));
    children
}

pub fn build_current_hero(
    current: &WeatherCurrent,
    units: &WeatherUnits,
    city: Option<&str>,
) -> Component {
    let temp = convert_temp(current.temp_c, *units);
    let unit = units.sdui_unit();
    let description = json!(format!("i18n:{}", current.description_key));

    let mut text_stack = Stack::new().gap(json!(4)).child(
        Text::new()
            .text(json!(format_temp_label(temp, unit, false)))
            .variant(json!("display"))
            .tone(tone_for_temp_c(current.temp_c)),
    );
    text_stack = text_stack.child(Text::new().text(description).variant(json!("caption")));
    if let Some(city) = city {
        text_stack = text_stack.child(Text::new().text(json!(city)).variant(json!("caption")));
    }

    Component::Stack(
        Stack::new()
            .direction(json!("horizontal"))
            .gap(json!(16))
            .child(
                Icon::new()
                    .name(json!(icon_name_for_condition(&current.condition)))
                    .size(json!(56)),
            )
            .child(Component::Stack(text_stack)),
    )
}

pub fn uv_warning_badge(current: &WeatherCurrent) -> Option<Component> {
    if !is_uv_high(current.uv_index) {
        return None;
    }
    let uv_key = current
        .uv_index
        .map(uv_label_key)
        .unwrap_or("weather.uv.high");
    Some(Component::Badge(
        Badge::new()
            .label(json!(format!("i18n:{uv_key}")))
            .tone(Tone::Warning),
    ))
}

fn build_forecast_day_column(
    day: &ForecastDayView,
    unit: &str,
    units: &WeatherUnits,
    locale: &str,
) -> Component {
    let display_temp = convert_temp(day.display_temp_c, *units);
    Component::Stack(
        Stack::new()
            .gap(json!(6))
            .child(
                Text::new()
                    .text(json!(format_day_strip_label(&day.date, locale)))
                    .variant(json!("caption")),
            )
            .child(
                Icon::new()
                    .name(json!(icon_name_for_condition(&day.condition)))
                    .size(json!(28)),
            )
            .child(
                Text::new()
                    .text(json!(format_temp_label(display_temp, unit, false)))
                    .variant(json!("caption"))
                    .emphasis(Emphasis::Strong)
                    .tone(tone_for_temp_c(day.display_temp_c)),
            ),
    )
}

fn build_forecast_strip(
    forecast: &WeatherForecast,
    units: &WeatherUnits,
    locale: &str,
) -> Component {
    let unit = units.sdui_unit();
    let day_columns: Vec<Component> = forecast
        .days
        .iter()
        .map(|day| build_forecast_day_column(day, unit, units, locale))
        .collect();

    Component::Grid(
        Grid::new()
            .columns(json!(5))
            .gap(json!(6))
            .children(day_columns),
    )
}
