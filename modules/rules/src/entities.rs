//! Persistent entities declared for Atlas migrations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Per-property house rules content (structured JSON per locale).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[portaki_sdk::entity(schema_version = 1)]
pub struct RulesContent {
    pub id: Uuid,
    pub content_fr: String,
    pub content_en: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
