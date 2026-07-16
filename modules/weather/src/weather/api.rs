//! OpenWeather connector calls and domain mapping.

use std::sync::atomic::{AtomicUsize, Ordering};

use chrono::{DateTime, Utc};
use portaki_connectors::open_weather::{
    CurrentArgs, CurrentWeather, ForecastArgs, ForecastDay, ForecastResponse, OpenWeather,
};
use portaki_sdk::prelude::*;

use crate::entities::WeatherUnits;
use crate::weather::condition::{description_key_for_condition, estimate_uv_index};
use crate::weather::format::weekday_key_for_date;
use crate::weather::model::{ForecastDayView, WeatherCurrent, WeatherForecast};

/// Counts OpenWeather `current` connector calls in unit tests.
pub static CONNECTOR_CURRENT_CALLS: AtomicUsize = AtomicUsize::new(0);

/// Counts OpenWeather `forecast` connector calls in unit tests.
pub static CONNECTOR_FORECAST_CALLS: AtomicUsize = AtomicUsize::new(0);

/// Returns true when pool or BYOK OpenWeather capability is granted.
pub fn has_open_weather(ctx: &Context) -> bool {
    ctx.has_capability(capability::external::OPEN_WEATHER_POOL)
        || ctx.has_capability(capability::external::OPEN_WEATHER_BYOK)
}

/// Fetches current weather from the OpenWeather connector.
pub fn fetch_current_from_api(lat: f64, lng: f64) -> Result<CurrentWeather> {
    CONNECTOR_CURRENT_CALLS.fetch_add(1, Ordering::SeqCst);
    OpenWeather::current(&CurrentArgs { lat, lng })
}

/// Fetches forecast from the OpenWeather connector.
pub fn fetch_forecast_from_api(lat: f64, lng: f64, days: u8) -> Result<ForecastResponse> {
    CONNECTOR_FORECAST_CALLS.fetch_add(1, Ordering::SeqCst);
    OpenWeather::forecast(&ForecastArgs { lat, lng, days })
}

/// Converts API current weather into module snapshot.
pub fn map_current(
    api: CurrentWeather,
    units: WeatherUnits,
    fetched_at: DateTime<Utc>,
) -> WeatherCurrent {
    WeatherCurrent {
        temp_c: api.temp_c,
        condition: api.condition.clone(),
        humidity: api.humidity,
        uv_index: estimate_uv_index(&api.condition),
        wind_speed_ms: api.wind_speed_ms,
        city_name: api.city_name,
        feels_like_c: api.feels_like_c,
        pressure_hpa: api.pressure_hpa,
        cloud_pct: api.cloud_pct,
        description_key: description_key_for_condition(&api.condition),
        units,
        fetched_at,
    }
}

/// Converts API forecast into guest-facing rows (max 5 days).
pub fn map_forecast(
    api: ForecastResponse,
    units: WeatherUnits,
    fetched_at: DateTime<Utc>,
) -> WeatherForecast {
    let days = api.days.into_iter().take(5).map(map_forecast_day).collect();
    WeatherForecast {
        days,
        city_name: api.city_name,
        units,
        fetched_at,
    }
}

fn map_forecast_day(day: ForecastDay) -> ForecastDayView {
    let midpoint_c = (day.min_c + day.max_c) / 2.0;
    ForecastDayView {
        date: day.date.clone(),
        weekday_key: weekday_key_for_date(&day.date),
        min_c: day.min_c,
        max_c: day.max_c,
        condition: day.condition,
        display_temp_c: midpoint_c,
        precip_chance_pct: day.precip_chance_pct,
        humidity_avg: day.humidity_avg,
        wind_speed_ms_max: day.wind_speed_ms_max,
    }
}
