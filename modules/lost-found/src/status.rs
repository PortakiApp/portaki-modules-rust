//! Report status wire values (host workflow pills).

use portaki_sdk::prelude::*;

/// Allowed status values on the wire.
///
/// | Wire          | FR (UI)        | EN              |
/// |---------------|----------------|-----------------|
/// | `to_collect`  | À récupérer    | To collect      |
/// | `sent`        | Envoyé         | Sent            |
/// | `returned`    | Récupéré       | Returned        |
pub const WIRE_VALUES: &[&str] = &["to_collect", "sent", "returned"];

/// Default when host omits status — « À récupérer ».
pub const DEFAULT: &str = "to_collect";

/// Validates and normalizes a status string from the form / command.
pub fn parse_status(raw: &str) -> Result<String> {
    let trimmed = raw.trim();
    if WIRE_VALUES.contains(&trimmed) {
        return Ok(trimmed.to_string());
    }
    Err(PortakiError::Host(format!("invalid_status:{trimmed}")))
}

/// Resolves optional status — empty / missing → [`DEFAULT`].
pub fn parse_status_or_default(raw: Option<&str>) -> Result<String> {
    match raw.map(str::trim).filter(|s| !s.is_empty()) {
        None => Ok(DEFAULT.to_string()),
        Some(value) => parse_status(value),
    }
}

/// i18n key for a stored status wire value.
#[allow(dead_code)]
pub fn status_label_key(wire: &str) -> &'static str {
    match wire {
        "sent" => "status.sent",
        "returned" => "status.returned",
        _ => "status.to_collect",
    }
}
