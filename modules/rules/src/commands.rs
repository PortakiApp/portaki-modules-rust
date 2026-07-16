//! Module commands — save house rules content.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::content::RulesPayload;
use crate::store;

/// Arguments for `saveContent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveContentArgs {
    /// Structured JSON for French (`{ "items": [ { "icon", "title", "subtitle" } ] }`).
    #[serde(default)]
    pub content_fr: String,
    /// Structured JSON for English.
    #[serde(default)]
    pub content_en: String,
}

#[portaki_sdk::command(name = "saveContent")]
pub fn save_content(_ctx: Context, args: SaveContentArgs) -> Result<()> {
    // Normalize invalid JSON to empty payloads so guests never see TipTap leftovers.
    let fr = RulesPayload::parse(&args.content_fr)
        .to_json_string()
        .map_err(|e| PortakiError::Host(format!("content_fr: {e}")))?;
    let en = RulesPayload::parse(&args.content_en)
        .to_json_string()
        .map_err(|e| PortakiError::Host(format!("content_en: {e}")))?;
    let _ = store::save_content_row(fr, en)?;
    Ok(())
}
