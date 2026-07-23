//! Guest empty / fallback surfaces.

use portaki_sdk::host::{log, module};
use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::EmptyState;
use portaki_sdk::sdui::surface::Surface;

use crate::content::MODULE_ICON;

pub fn empty_inactive_state(surface_id: SurfaceId) -> Surface {
    Surface::new(
        EmptyState::new()
            .title("i18n:module.status.inactive.title")
            .description("i18n:module.status.inactive.description")
            .icon(MODULE_ICON),
    )
    .with_id(surface_id)
}

pub fn empty_runtime_error_state(surface_id: SurfaceId) -> Surface {
    Surface::new(
        EmptyState::new()
            .title("i18n:module.status.inactive.title")
            .icon(MODULE_ICON),
    )
    .with_id(surface_id)
}

pub fn log_render_failure(surface_id: SurfaceId, error: &PortakiError) {
    let mut fields = log::Fields::new();
    fields.insert("surfaceId", &surface_id);
    fields.insert("error", &error.to_string());
    let _ = log::error("train_guest_render_failed", &fields);
}

/// Returns `Some(surface)` with an inactive empty state when the module is not
/// enabled on this property/workspace — `None` means render normally.
pub fn empty_state_if_module_not_ready(surface_id: SurfaceId) -> Result<Option<Surface>> {
    let status = module::status()?;
    if !status.workspace_enabled || !status.active {
        return Ok(Some(empty_inactive_state(surface_id)));
    }
    Ok(None)
}
