//! Persistent entities declared for Atlas migrations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Temperature unit preference stored with cached rows.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum WeatherUnits {
    /// Celsius (default).
    #[default]
    Celsius,
    /// Fahrenheit.
    Fahrenheit,
}

impl WeatherUnits {
    /// OpenWeather temperature unit label for SDUI.
    pub fn sdui_unit(&self) -> &'static str {
        match self {
            WeatherUnits::Celsius => "celsius",
            WeatherUnits::Fahrenheit => "fahrenheit",
        }
    }
}

/// Cached OpenWeather payload for a coordinate pair.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[portaki_sdk::entity(schema_version = 1)]
pub struct WeatherCache {
    pub id: Uuid,
    pub lat: f64,
    pub lng: f64,
    pub current_json: String,
    pub forecast_json: String,
    pub units: WeatherUnits,
    pub fetched_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[portaki_sdk::entity_indexes(WeatherCache)]
#[allow(dead_code)]
pub const WEATHER_CACHE_INDEXES: &[&str] = &["lat", "lng"];
