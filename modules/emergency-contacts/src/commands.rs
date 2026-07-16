//! Module commands — configuration persistence.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{save_config, ModuleConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigArgs {
    #[serde(default)]
    pub contacts_json: String,
    #[serde(default)]
    pub host_visible_phone: String,
}

#[portaki_sdk::command(name = "updateConfig")]
pub fn update_config(_ctx: Context, args: UpdateConfigArgs) -> Result<()> {
    save_config(&ModuleConfig {
        contacts_json: args.contacts_json,
        host_visible_phone: args.host_visible_phone,
    })
}
