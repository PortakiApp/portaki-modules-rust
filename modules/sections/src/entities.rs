//! Persistent entities declared for Atlas migrations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Editorial section row (ordering only).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[portaki_sdk::entity(schema_version = 1)]
pub struct SectionItem {
    pub id: Uuid,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[portaki_sdk::entity_indexes(SectionItem)]
#[allow(dead_code)]
pub const SECTION_ITEM_INDEXES: &[&str] = &["sort_order"];

/// Localized title + markdown body for a section.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[portaki_sdk::entity(schema_version = 1)]
pub struct SectionItemLocale {
    pub id: Uuid,
    pub section_id: Uuid,
    pub lang: String,
    pub title: String,
    pub body_markdown: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[portaki_sdk::entity_indexes(SectionItemLocale)]
#[allow(dead_code)]
pub const SECTION_ITEM_LOCALE_INDEXES: &[&str] = &["section_id", "lang"];
