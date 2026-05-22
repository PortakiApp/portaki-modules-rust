//! Module queries — current conditions and multi-day forecast.

use portaki_sdk::host::time;
use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::cache;
use crate::config::load_config;
use crate::weather::{
    fetch_current_from_api, fetch_forecast_from_api, has_open_weather, map_current, map_forecast,
    WeatherCurrent, WeatherForecast,
};

/// Arguments for `getCurrent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCurrentArgs {
    /// Optional latitude override (defaults to property coordinates).
    pub lat: Option<f64>,
    /// Optional longitude override.
    pub lng: Option<f64>,
}

/// Arguments for `getForecast`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetForecastArgs {
    /// Optional latitude override.
    pub lat: Option<f64>,
    /// Optional longitude override.
    pub lng: Option<f64>,
    /// Number of forecast days (default 5).
    pub days: Option<u8>,
}

#[portaki_sdk::query(name = "getCurrent")]
pub fn get_current(ctx: Context, args: GetCurrentArgs) -> Result<WeatherCurrent> {
    if !has_open_weather(&ctx) {
        return Err(PortakiError::CapabilityNotAvailable(
            "external.open-weather".to_string(),
        ));
    }

    let config = load_config()?;
    let lat = args.lat.unwrap_or(ctx.property.lat);
    let lng = args.lng.unwrap_or(ctx.property.lng);
    let now = time::now()?;

    if let Some(cached) = cache::read_current(lat, lng, config.units, now)? {
        return Ok(cached);
    }

    let api = fetch_current_from_api(lat, lng)?;
    let current = map_current(api, config.units, now);

    let forecast_api = fetch_forecast_from_api(lat, lng, 5)?;
    let forecast = map_forecast(forecast_api, config.units, now);
    cache::store_current(lat, lng, config.units, &current, &forecast.days)?;

    Ok(current)
}

#[portaki_sdk::query(name = "getForecast")]
pub fn get_forecast(ctx: Context, args: GetForecastArgs) -> Result<WeatherForecast> {
    if !has_open_weather(&ctx) {
        return Err(PortakiError::CapabilityNotAvailable(
            "external.open-weather".to_string(),
        ));
    }

    let config = load_config()?;
    let lat = args.lat.unwrap_or(ctx.property.lat);
    let lng = args.lng.unwrap_or(ctx.property.lng);
    let days = args.days.unwrap_or(5);
    let now = time::now()?;

    if let Some(cached) = cache::read_forecast(lat, lng, config.units, now)? {
        return Ok(cached);
    }

    let forecast_api = fetch_forecast_from_api(lat, lng, days)?;
    let forecast = map_forecast(forecast_api, config.units, now);

    let current_api = fetch_current_from_api(lat, lng)?;
    let current = map_current(current_api, config.units, now);
    cache::store_current(lat, lng, config.units, &current, &forecast.days)?;

    Ok(forecast)
}
