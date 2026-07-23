//! Persistent entities declared for Atlas migrations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// One guest lost/found report for a stay (many per stay allowed).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[portaki_sdk::entity(schema_version = 1)]
pub struct LostFoundReport {
    pub id: Uuid,
    pub stay_id: Uuid,
    pub kind: String,
    pub item_description: String,
    pub contact_hint: Option<String>,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[portaki_sdk::entity_indexes(LostFoundReport)]
#[allow(dead_code)]
pub const LOST_FOUND_REPORT_INDEXES: &[&str] = &["stay_id"];
