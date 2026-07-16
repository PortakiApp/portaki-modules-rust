//! Domain snapshots and cache payloads.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::WeatherUnits;

/// TTL for current conditions cache (1 hour).
pub const CURRENT_CACHE_TTL_SECS: i64 = 3600;
/// TTL for forecast cache (6 hours).
pub const FORECAST_CACHE_TTL_SECS: i64 = 21_600;

/// Snapshot returned by `getCurrent`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeatherCurrent {
    pub temp_c: f64,
    pub condition: String,
    pub humidity: u8,
    pub uv_index: Option<f64>,
    pub wind_speed_ms: Option<f64>,
    pub city_name: Option<String>,
    pub feels_like_c: Option<f64>,
    pub pressure_hpa: Option<u16>,
    pub cloud_pct: Option<u8>,
    pub description_key: String,
    pub units: WeatherUnits,
    pub fetched_at: DateTime<Utc>,
}

/// One forecast day for guests and queries.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForecastDayView {
    pub date: String,
    pub weekday_key: String,
    pub min_c: f64,
    pub max_c: f64,
    pub condition: String,
    /// Midpoint temperature in Celsius (convert at render time).
    pub display_temp_c: f64,
    pub precip_chance_pct: Option<u8>,
    pub humidity_avg: Option<u8>,
    pub wind_speed_ms_max: Option<f64>,
}

/// Forecast bundle returned by `getForecast`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeatherForecast {
    pub days: Vec<ForecastDayView>,
    pub city_name: Option<String>,
    pub units: WeatherUnits,
    pub fetched_at: DateTime<Utc>,
}

/// Serialized payload stored in `WeatherCache.current_json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedCurrentPayload {
    pub temp_c: f64,
    pub condition: String,
    pub humidity: u8,
    pub uv_index: Option<f64>,
    #[serde(default)]
    pub wind_speed_ms: Option<f64>,
    #[serde(default)]
    pub city_name: Option<String>,
    #[serde(default)]
    pub feels_like_c: Option<f64>,
    #[serde(default)]
    pub pressure_hpa: Option<u16>,
    #[serde(default)]
    pub cloud_pct: Option<u8>,
}

impl CachedCurrentPayload {
    pub fn from_current(current: &WeatherCurrent) -> Self {
        Self {
            temp_c: current.temp_c,
            condition: current.condition.clone(),
            humidity: current.humidity,
            uv_index: current.uv_index,
            wind_speed_ms: current.wind_speed_ms,
            city_name: current.city_name.clone(),
            feels_like_c: current.feels_like_c,
            pressure_hpa: current.pressure_hpa,
            cloud_pct: current.cloud_pct,
        }
    }

    pub fn into_current(
        self,
        units: WeatherUnits,
        fetched_at: DateTime<Utc>,
        description_key: String,
    ) -> WeatherCurrent {
        WeatherCurrent {
            temp_c: self.temp_c,
            condition: self.condition,
            humidity: self.humidity,
            uv_index: self.uv_index,
            wind_speed_ms: self.wind_speed_ms,
            city_name: self.city_name,
            feels_like_c: self.feels_like_c,
            pressure_hpa: self.pressure_hpa,
            cloud_pct: self.cloud_pct,
            description_key,
            units,
            fetched_at,
        }
    }
}

/// Serialized payload stored in `WeatherCache.forecast_json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedForecastPayload {
    pub days: Vec<ForecastDayView>,
    #[serde(default)]
    pub city_name: Option<String>,
}

impl CachedForecastPayload {
    pub fn from_forecast(forecast: &WeatherForecast, fallback_city: Option<String>) -> Self {
        Self {
            days: forecast.days.clone(),
            city_name: forecast.city_name.clone().or(fallback_city),
        }
    }

    pub fn into_forecast(self, units: WeatherUnits, fetched_at: DateTime<Utc>) -> WeatherForecast {
        WeatherForecast {
            days: self.days,
            city_name: self.city_name,
            units,
            fetched_at,
        }
    }
}
