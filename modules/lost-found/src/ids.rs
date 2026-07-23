//! Typed surface / operation catalogs for this module.

use portaki_sdk::prelude::*;

define_surface_ids! {
    HOME_CARD = "home.card",
    HOST_MAIN = "main",
}

define_operation_names! {
    LIST_FOR_STAY = "listForStay",
    LIST_RECENT = "listRecent",
    SUBMIT = "submit",
    UPDATE_CONFIG = "updateConfig",
    EMAIL_CONTEXT = "emailContext",
}

define_event_types! {
    SUBMITTED = "lost-found.submitted",
}

/// Catalog module id (`lost-found`).
pub fn module_id() -> ModuleId {
    ModuleId::from_static("lost-found")
}
