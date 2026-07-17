//! OpenWeather condition → i18n keys, icons, UV heuristics.

/// Maps a condition code to an i18n description key.
pub fn description_key_for_condition(condition: &str) -> String {
    match classify(condition) {
        ConditionKind::Clear => "weather.description.sunny",
        ConditionKind::Cloudy => "weather.description.cloudy",
        ConditionKind::Rain => "weather.description.rainy",
        ConditionKind::Snow => "weather.description.snowy",
        ConditionKind::Storm => "weather.description.stormy",
        ConditionKind::Fog => "weather.description.foggy",
    }
    .to_string()
}

/// Maps OpenWeather condition → Lucide / pk-icon name.
pub fn icon_name_for_condition(condition: &str) -> &'static str {
    match classify(condition) {
        ConditionKind::Storm => "cloud-lightning",
        ConditionKind::Snow => "cloud-snow",
        ConditionKind::Rain => "cloud-rain",
        ConditionKind::Fog => "cloud-fog",
        ConditionKind::Clear => "sun",
        ConditionKind::Cloudy => "cloud-sun",
    }
}

/// UV badge i18n key from index.
pub fn uv_label_key(uv_index: f64) -> &'static str {
    if uv_index < 3.0 {
        "weather.uv.low"
    } else if uv_index < 6.0 {
        "weather.uv.moderate"
    } else if uv_index < 8.0 {
        "weather.uv.high"
    } else {
        "weather.uv.extreme"
    }
}

/// Rough UV estimate when the connector does not provide one.
pub fn estimate_uv_index(condition: &str) -> Option<f64> {
    match classify(condition) {
        ConditionKind::Clear => Some(7.5),
        ConditionKind::Cloudy => Some(4.0),
        _ => Some(2.0),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConditionKind {
    Clear,
    Cloudy,
    Rain,
    Snow,
    Storm,
    Fog,
}

fn classify(condition: &str) -> ConditionKind {
    let normalized = condition.to_ascii_lowercase();
    if normalized.contains("storm") || normalized.contains("thunder") {
        ConditionKind::Storm
    } else if normalized.contains("snow") {
        ConditionKind::Snow
    } else if normalized.contains("rain") || normalized.contains("drizzle") {
        ConditionKind::Rain
    } else if normalized.contains("fog") || normalized.contains("mist") {
        ConditionKind::Fog
    } else if normalized.contains("clear") || normalized.contains("sun") {
        ConditionKind::Clear
    } else {
        ConditionKind::Cloudy
    }
}
