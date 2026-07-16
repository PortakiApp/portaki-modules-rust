//! Module commands — configuration persistence.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{save_config, ModuleConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigArgs {
    #[serde(default)]
    pub spots_json: String,
    #[serde(default)]
    pub disclaimer: String,
}

#[portaki_sdk::command(name = "updateConfig")]
pub fn update_config(_ctx: Context, args: UpdateConfigArgs) -> Result<()> {
    save_config(&ModuleConfig {
        spots_json: args.spots_json,
        disclaimer: args.disclaimer,
    })
}
