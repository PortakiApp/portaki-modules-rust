//! Module commands — guest submit, host submitFound / updateStatus, host config.

use portaki_sdk::host::events;
use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::{save_config, ModuleConfig};
use crate::description;
use crate::kind;
use crate::status;
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

#[portaki_sdk::wire(serialize)]
struct HostFoundPayload {
    property_id: Uuid,
    stay_id: Uuid,
    item_description: String,
    status: String,
    report_id: Uuid,
}

/// Arguments for guest `submit`.
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
    let stay_id = require_guest_stay_id(&ctx)?;
    let kind = kind::parse_kind(&args.kind)?;
    let item_description = require_description(&args.item_description)?;
    let contact_hint = normalize_optional(args.contact_hint);
    let details = normalize_optional(args.details);

    let _ = storage::create(
        stay_id,
        kind.clone(),
        item_description.clone(),
        contact_hint.clone(),
        details.clone(),
        status::DEFAULT.to_string(),
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

/// Arguments for host `submitFound` — one report per stay (shared description/status).
#[portaki_sdk::wire]
pub struct SubmitFoundArgs {
    /// Target stays (property-scoped). Empty when only [`Self::stay_id`] is set.
    #[serde(default)]
    pub stay_ids: Vec<Uuid>,
    /// Convenience single stay (merged into `stay_ids`).
    #[serde(default)]
    pub stay_id: Option<Uuid>,
    /// TipTap JSON or plain text — stored as-is; email gets plain extract.
    pub description: String,
    /// Wire status — default `to_collect` (« À récupérer »).
    #[serde(default)]
    pub status: Option<String>,
}

#[portaki_sdk::command(name = "submitFound")]
pub fn submit_found(ctx: Context, args: SubmitFoundArgs) -> Result<()> {
    if ctx.guest.is_some() {
        return Err(PortakiError::Host("host_only".to_string()));
    }

    let stay_ids = resolve_stay_ids(&args)?;
    let description = require_description(&args.description)?;
    let status = status::parse_status_or_default(args.status.as_deref())?;
    let plain = description::to_plain_text(&description);
    if plain.is_empty() {
        return Err(PortakiError::Host("description_required".to_string()));
    }

    for stay_id in stay_ids {
        let report = storage::create(
            stay_id,
            "found".to_string(),
            description.clone(),
            None,
            None,
            status.clone(),
        )?;

        events::emit(
            crate::ids::HOST_FOUND,
            &HostFoundPayload {
                property_id: ctx.property_id,
                stay_id,
                item_description: plain.clone(),
                status: status.clone(),
                report_id: report.id,
            },
        )?;
    }

    Ok(())
}

/// Arguments for host `updateStatus` — change workflow status after create.
#[portaki_sdk::wire]
pub struct UpdateStatusArgs {
    pub report_id: Uuid,
    pub status: String,
}

#[portaki_sdk::command(name = "updateStatus")]
pub fn update_status(ctx: Context, args: UpdateStatusArgs) -> Result<()> {
    if ctx.guest.is_some() {
        return Err(PortakiError::Host("host_only".to_string()));
    }

    let status = status::parse_status(&args.status)?;
    let _ = storage::update_status(args.report_id, status)?;
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

fn resolve_stay_ids(args: &SubmitFoundArgs) -> Result<Vec<Uuid>> {
    let mut ids = args.stay_ids.clone();
    if let Some(stay_id) = args.stay_id {
        if !ids.contains(&stay_id) {
            ids.push(stay_id);
        }
    }
    ids.sort_unstable();
    ids.dedup();
    if ids.is_empty() {
        return Err(PortakiError::Host("stay_ids_required".to_string()));
    }
    Ok(ids)
}

fn require_description(raw: &str) -> Result<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || description::to_plain_text(trimmed).is_empty() {
        return Err(PortakiError::Host("description_required".to_string()));
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

fn require_guest_stay_id(ctx: &Context) -> Result<Uuid> {
    ctx.guest
        .as_ref()
        .map(|guest| guest.session_id)
        .ok_or_else(|| PortakiError::Host("stay_id_required".to_string()))
}
