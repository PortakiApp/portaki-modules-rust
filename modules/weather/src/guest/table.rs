//! 5-day forecast table for the explore sheet.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::common::{Emphasis, Tone};
use portaki_sdk::sdui::primitives::{Grid, Icon, Text};
use serde_json::json;

use crate::entities::WeatherUnits;
use crate::weather::{
    convert_temp, format_day_strip_label, format_temp_label, format_wind_kmh, icon_name_for_condition,
    WeatherForecast,
};

use super::components::{metric_label, optional_pct, table_header_cell, table_value_cell};

/// Forecast table: day · icon · min · max · rain · humidity · wind (scrolls when wide).
pub fn build_forecast_table(
    forecast: &WeatherForecast,
    units: &WeatherUnits,
    locale: &str,
) -> Component {
    let unit = units.sdui_unit();
    let mut cells: Vec<Component> = vec![
        table_header_cell("i18n:weather.col.day"),
        table_header_cell(""),
        table_header_cell("i18n:weather.col.min"),
        table_header_cell("i18n:weather.col.max"),
        metric_label("cloud-rain", "i18n:weather.col.precip"),
        metric_label("droplets", "i18n:weather.col.humidity"),
        metric_label("wind", "i18n:weather.col.wind"),
    ];

    for day in &forecast.days {
        let min = format_temp_label(convert_temp(day.min_c, *units), unit, false);
        let max = format_temp_label(convert_temp(day.max_c, *units), unit, false);
        let precip = optional_pct(day.precip_chance_pct);
        let humidity = optional_pct(day.humidity_avg);
        let wind = day
            .wind_speed_ms_max
            .map(format_wind_kmh)
            .unwrap_or_else(|| "—".to_string());

        cells.push(Component::Text(
            Text::new()
                .text(json!(format_day_strip_label(&day.date, locale)))
                .variant(json!("caption"))
                .emphasis(Emphasis::Strong),
        ));
        cells.push(Component::Icon(
            Icon::new()
                .name(json!(icon_name_for_condition(&day.condition)))
                .size(json!(28)),
        ));
        cells.push(Component::Text(
            Text::new()
                .text(json!(min))
                .variant(json!("caption"))
                .tone(Tone::Success),
        ));
        cells.push(Component::Text(
            Text::new()
                .text(json!(max))
                .variant(json!("caption"))
                .tone(Tone::Warning),
        ));
        cells.push(table_value_cell(&precip));
        cells.push(table_value_cell(&humidity));
        cells.push(table_value_cell(&wind));
    }

    Component::Grid(Grid::new().columns(json!(7)).gap(json!(10)).children(cells))
}
