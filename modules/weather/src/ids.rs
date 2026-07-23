//! Typed surface / operation catalogs for this module.

use portaki_sdk::prelude::*;

define_surface_ids! {
    HOME_CARD = "home.card",
    EXPLORE_FORECAST = "explore.forecast",
    HOST_MAIN = "main",
}

define_operation_names! {
    EMAIL_CONTEXT = "emailContext",
    GET_CURRENT = "getCurrent",
    GET_FORECAST = "getForecast",
    REFRESH_FORECAST = "refreshForecast",
    UPDATE_CONFIG = "updateConfig",
}

/// Catalog module id (`weather`).
pub fn module_id() -> ModuleId {
    ModuleId::from_static("weather")
}
