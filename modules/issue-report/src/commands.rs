//! Module commands — submit issue report.

use portaki_sdk::host::events;
use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::category;
use crate::storage;

#[portaki_sdk::wire(serialize)]
#[derive(Serialize)]
struct SubmittedPayload {
    property_id: Uuid,
    category: String,
    summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    stay_id: Uuid,
}

/// Arguments for `submit`.
#[portaki_sdk::wire]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitArgs {
    pub category: String,
    pub summary: String,
    #[serde(default)]
    pub details: Option<String>,
}

#[portaki_sdk::command(name = "submit")]
pub fn submit(ctx: Context, args: SubmitArgs) -> Result<()> {
    let stay_id = require_stay_id(&ctx)?;
    let category = category::parse_category(&args.category)?;
    let summary = require_summary(&args.summary)?;
    let details = normalize_optional(args.details);

    let _ = storage::create(stay_id, category.clone(), summary.clone(), details.clone())?;

    events::emit(
        crate::ids::SUBMITTED,
        &SubmittedPayload {
            property_id: ctx.property_id,
            category,
            summary,
            details,
            stay_id,
        },
    )?;
    Ok(())
}

fn require_summary(raw: &str) -> Result<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(PortakiError::Host("summary_required".to_string()));
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
