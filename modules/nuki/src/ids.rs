//! Typed surface / operation catalogs for this module.

use portaki_sdk::prelude::*;

define_surface_ids! {
    HOST_MAIN = "main",
}

define_operation_names! {
    GET_CONFIG = "getConfig",
    UPDATE_CONFIG = "updateConfig",
}

/// Catalog module id (`nuki`).
pub fn module_id() -> ModuleId {
    ModuleId::from_static("nuki")
}
