//! Guest explore / bottom-sheet forecast surface.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{Divider, Stack, Text};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use super::body::{build_current_hero, uv_warning_badge};
use super::details::build_current_details;
use super::load::GuestWeatherData;
use super::table::build_forecast_table;

/// Body-only tree for the bottom sheet (shell supplies header chrome).
pub fn build_sheet_surface(data: &GuestWeatherData) -> Surface {
    let mut children = vec![build_current_hero(
        &data.current,
        &data.units,
        data.city.as_deref(),
    )];
    if let Some(badge) = uv_warning_badge(&data.current) {
        children.push(badge);
    }
    children.push(Component::Divider(Divider::new()));
    children.push(build_current_details(&data.current, &data.units));
    children.push(Component::Divider(Divider::new()));
    children.push(Component::Text(
        Text::new()
            .text(json!("i18n:explore.forecast.hint"))
            .variant(json!("title")),
    ));
    children.push(build_forecast_table(
        &data.forecast,
        &data.units,
        &data.locale,
    ));

    Surface::new(Stack::new().gap(json!(12)).children(children)).with_id("explore.forecast")
}
