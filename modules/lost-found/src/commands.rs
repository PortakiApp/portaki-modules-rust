//! Module commands — submit report and host config.

use portaki_sdk::host::events;
use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::{save_config, ModuleConfig};
use crate::kind;
use crate::storage;

#[portaki_sdk::wire(serialize)]
struct SubmittedPayload {
    property_id: Uuid,
    kind: String,
    item_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    stay_id: Uuid,
}

/// Arguments for `submit`.
#[portaki_sdk::wire]
pub struct SubmitArgs {
    pub kind: String,
    pub item_description: String,
    #[serde(default)]
    pub contact_hint: Option<String>,
    #[serde(default)]
    pub details: Option<String>,
}

#[portaki_sdk::command(name = "submit")]
pub fn submit(ctx: Context, args: SubmitArgs) -> Result<()> {
    let stay_id = require_stay_id(&ctx)?;
    let kind = kind::parse_kind(&args.kind)?;
    let item_description = require_item_description(&args.item_description)?;
    let contact_hint = normalize_optional(args.contact_hint);
    let details = normalize_optional(args.details);

    let _ = storage::create(
        stay_id,
        kind.clone(),
        item_description.clone(),
        contact_hint.clone(),
        details.clone(),
    )?;

    events::emit(
        crate::ids::SUBMITTED,
        &SubmittedPayload {
            property_id: ctx.property_id,
            kind,
            item_description,
            contact_hint,
            details,
            stay_id,
        },
    )?;
    Ok(())
}

/// Arguments for `updateConfig`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigArgs {
    #[serde(default)]
    pub host_note: String,
}

#[portaki_sdk::command(name = "updateConfig")]
pub fn update_config(_ctx: Context, args: UpdateConfigArgs) -> Result<()> {
    let trimmed = args.host_note.trim();
    let host_note = if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    };
    save_config(&ModuleConfig { host_note })
}

fn require_item_description(raw: &str) -> Result<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(PortakiError::Host("item_description_required".to_string()));
    }
    Ok(trimmed.to_string())
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value.and_then(|raw| {
        let trimmed = raw.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

fn require_stay_id(ctx: &Context) -> Result<Uuid> {
    ctx.guest
        .as_ref()
        .map(|guest| guest.session_id)
        .ok_or_else(|| PortakiError::Host("stay_id_required".to_string()))
}
