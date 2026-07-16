//! Module commands — configuration persistence.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{save_config, ModuleConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigArgs {
    #[serde(default)]
    pub steps_json: String,
    #[serde(default)]
    pub parking_map_url: String,
    #[serde(default)]
    pub arrival_video_url: String,
    #[serde(default)]
    pub global_note: String,
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub gate_code: String,
    #[serde(default)]
    pub keybox_code: String,
    #[serde(default)]
    pub parking_info: String,
}

#[portaki_sdk::command(name = "updateConfig")]
pub fn update_config(_ctx: Context, args: UpdateConfigArgs) -> Result<()> {
    save_config(&ModuleConfig {
        steps_json: args.steps_json,
        parking_map_url: args.parking_map_url,
        arrival_video_url: args.arrival_video_url,
        global_note: args.global_note,
        address: args.address,
        gate_code: args.gate_code,
        keybox_code: args.keybox_code,
        parking_info: args.parking_info,
    })
}
