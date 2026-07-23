//! Module commands — submit pre-arrival form.

use portaki_sdk::host::events;
use portaki_sdk::prelude::*;
use uuid::Uuid;

use crate::storage;

#[portaki_sdk::wire(serialize)]
struct CompletedPayload {
    arrival_time_estimated: Option<String>,
    guest_occasion: Option<String>,
    guest_allergies: Option<String>,
    message_to_host: Option<String>,
}

/// Arguments for `submit`.
#[portaki_sdk::wire]
pub struct SubmitArgs {
    pub arrival_time_estimated: Option<String>,
    pub guest_occasion: Option<String>,
    pub guest_allergies: Option<String>,
    pub message_to_host: Option<String>,
}

#[portaki_sdk::command(name = "submit")]
pub fn submit(ctx: Context, args: SubmitArgs) -> Result<()> {
    let stay_id = require_stay_id(&ctx)?;
    let arrival_time = normalize(args.arrival_time_estimated);
    let occasion = normalize(args.guest_occasion);
    let allergies = normalize(args.guest_allergies);
    let message = normalize(args.message_to_host);

    let _ = storage::upsert(
        stay_id,
        arrival_time.clone(),
        occasion.clone(),
        allergies.clone(),
        message.clone(),
    )?;

    events::emit(
        crate::ids::COMPLETED,
        &CompletedPayload {
            arrival_time_estimated: arrival_time,
            guest_occasion: occasion,
            guest_allergies: allergies,
            message_to_host: message,
        },
    )?;
    Ok(())
}

fn normalize(value: Option<String>) -> Option<String> {
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
