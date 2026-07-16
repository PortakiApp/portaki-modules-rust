//! Module commands — save appliance guide.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::content::AppliancesPayload;
use crate::store;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveContentArgs {
    #[serde(default)]
    pub content_fr: String,
    #[serde(default)]
    pub content_en: String,
}

#[portaki_sdk::command(name = "saveContent")]
pub fn save_content(_ctx: Context, args: SaveContentArgs) -> Result<()> {
    let fr = AppliancesPayload::parse(&args.content_fr)
        .to_json_string()
        .map_err(|e| PortakiError::Host(format!("content_fr: {e}")))?;
    let en = AppliancesPayload::parse(&args.content_en)
        .to_json_string()
        .map_err(|e| PortakiError::Host(format!("content_en: {e}")))?;
    let _ = store::save_content_row(fr, en)?;
    Ok(())
}
