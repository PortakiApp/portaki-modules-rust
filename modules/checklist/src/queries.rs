//! Module queries — checklist items and stay completions.

use portaki_sdk::prelude::*;
use uuid::Uuid;

use crate::entities::ChecklistItem;
use crate::storage;

/// Public item DTO returned by `listItems`.
#[portaki_sdk::wire]
#[derive(PartialEq, Eq)]
pub struct ChecklistItemDto {
    pub id: Uuid,
    pub label_fr: String,
    pub label_en: String,
    pub sort_order: i32,
}

impl From<ChecklistItem> for ChecklistItemDto {
    fn from(value: ChecklistItem) -> Self {
        Self {
            id: value.id,
            label_fr: value.label_fr,
            label_en: value.label_en,
            sort_order: value.sort_order,
        }
    }
}

#[portaki_sdk::query(name = "listItems")]
pub fn list_items(_ctx: Context) -> Result<Vec<ChecklistItemDto>> {
    Ok(storage::list_items()?
        .into_iter()
        .map(ChecklistItemDto::from)
        .collect())
}

#[portaki_sdk::query(name = "listCompletions")]
pub fn list_completions(ctx: Context) -> Result<Vec<Uuid>> {
    let stay_id = require_stay_id(&ctx)?;
    storage::list_completed_item_ids(stay_id)
}

fn require_stay_id(ctx: &Context) -> Result<Uuid> {
    ctx.guest
        .as_ref()
        .map(|guest| guest.session_id)
        .ok_or_else(|| PortakiError::Host("stay_id_required".to_string()))
}
