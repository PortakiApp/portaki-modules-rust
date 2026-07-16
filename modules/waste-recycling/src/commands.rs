//! Module commands — configuration persistence.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{save_config, ModuleConfig};

/// Arguments for `updateConfig`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigArgs {
    #[serde(default)]
    pub bins_json: String,
    #[serde(default)]
    pub collection_schedule: String,
}

#[portaki_sdk::command(name = "updateConfig")]
pub fn update_config(_ctx: Context, args: UpdateConfigArgs) -> Result<()> {
    save_config(&ModuleConfig {
        bins_json: args.bins_json,
        collection_schedule: args.collection_schedule,
    })
}
