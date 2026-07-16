//! Module commands — configuration persistence.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{save_config, ModuleConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigArgs {
    #[serde(default)]
    pub facilities_json: String,
    #[serde(default)]
    pub general_note: String,
}

#[portaki_sdk::command(name = "updateConfig")]
pub fn update_config(_ctx: Context, args: UpdateConfigArgs) -> Result<()> {
    save_config(&ModuleConfig {
        facilities_json: args.facilities_json,
        general_note: args.general_note,
    })
}
