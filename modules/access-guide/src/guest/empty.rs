//! Guest empty / fallback surfaces.

use portaki_sdk::host::{log, module};
use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{EmptyState, Text};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

pub fn empty_content_state(surface_id: &str) -> Surface {
    Surface::new(
        EmptyState::new()
            .title(json!("i18n:guest.empty.title"))
            .description(json!("i18n:guest.empty.description"))
            .icon(json!("car"))
            .child(
                Text::new()
                    .text(json!("i18n:guest.empty.hint"))
                    .variant(json!("body")),
            ),
    )
    .with_id(surface_id)
}

pub fn empty_runtime_error_state(surface_id: &str) -> Surface {
    Surface::new(
        EmptyState::new()
            .title(json!("i18n:guest.error.title"))
            .description(json!("i18n:guest.error.description"))
            .icon(json!("car"))
            .child(
                Text::new()
                    .text(json!("i18n:guest.error.hint"))
                    .variant(json!("body")),
            ),
    )
    .with_id(surface_id)
}

fn empty_config_state(surface_id: &str) -> Surface {
    Surface::new(
        EmptyState::new()
            .title(json!("i18n:module.status.incomplete.title"))
            .description(json!("i18n:module.status.incomplete.description"))
            .icon(json!("sliders"))
            .child(
                Text::new()
                    .text(json!("i18n:module.status.incomplete.hint"))
                    .variant(json!("body")),
            ),
    )
    .with_id(surface_id)
}

fn empty_inactive_state(surface_id: &str) -> Surface {
    Surface::new(
        EmptyState::new()
            .title(json!("i18n:module.status.inactive.title"))
            .description(json!("i18n:module.status.inactive.description"))
            .icon(json!("car")),
    )
    .with_id(surface_id)
}

pub fn log_render_failure(surface_id: &str, error: &PortakiError) {
    let mut fields = log::Fields::new();
    fields.insert("surfaceId", &surface_id);
    fields.insert("error", &error.to_string());
    let _ = log::error("access_guide_guest_render_failed", &fields);
}

pub fn empty_state_if_module_not_ready(surface_id: &str) -> Result<Option<Surface>> {
    let status = module::status()?;
    if !status.workspace_enabled || !status.active {
        return Ok(Some(empty_inactive_state(surface_id)));
    }
    if status.incomplete {
        return Ok(Some(empty_config_state(surface_id)));
    }
    Ok(None)
}
