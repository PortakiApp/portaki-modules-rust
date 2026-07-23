//! Module commands — configuration and Portaki review submit.

use portaki_sdk::host;
use portaki_sdk::host::events;
use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{load_config, save_config, Localized, ModuleConfig, ReviewChannel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigArgs {
    #[serde(default)]
    pub review_channel: String,
    #[serde(default = "default_true")]
    pub show_qr_code: bool,
    #[serde(default)]
    pub airbnb_review_url: String,
    #[serde(default)]
    pub thank_you_message: String,
}

fn default_true() -> bool {
    true
}

#[portaki_sdk::command(name = "updateConfig")]
pub fn update_config(ctx: Context, args: UpdateConfigArgs) -> Result<()> {
    let lang = Localized::lang_code(&ctx.locale);
    let existing = load_config().unwrap_or_default();
    let mut thank_you_message = existing.thank_you_message;
    thank_you_message.set(&lang, args.thank_you_message.trim().to_string());
    save_config(&ModuleConfig {
        review_channel: ReviewChannel::parse(&args.review_channel),
        show_qr_code: args.show_qr_code,
        airbnb_review_url: args.airbnb_review_url,
        thank_you_message,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitReviewArgs {
    pub rating: u8,
    #[serde(default)]
    pub comment: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReviewSubmittedPayload {
    property_id: Uuid,
    rating: u8,
    comment: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_name: Option<String>,
}

const REVIEWS_KEY: &str = "reviews";

#[portaki_sdk::command(name = "submitReview")]
pub fn submit_review(ctx: Context, args: SubmitReviewArgs) -> Result<()> {
    if !(1..=5).contains(&args.rating) {
        return Err(PortakiError::Host(format!(
            "rating must be 1-5, got {}",
            args.rating
        )));
    }

    let comment = args.comment.trim().to_string();
    let mut entries: Vec<SubmitReviewArgs> = host::kv::get(REVIEWS_KEY)?
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default();

    entries.push(SubmitReviewArgs {
        rating: args.rating,
        comment: comment.clone(),
    });

    let bytes = serde_json::to_vec(&entries)
        .map_err(|error| PortakiError::Storage(format!("reviews serialize: {error}")))?;
    host::kv::set(REVIEWS_KEY, &bytes, None)?;

    let guest_name = ctx
        .guest
        .as_ref()
        .and_then(|g| g.display_name.clone())
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty());

    events::emit(
        crate::ids::SUBMITTED,
        &ReviewSubmittedPayload {
            property_id: ctx.property_id,
            rating: args.rating,
            comment,
            guest_name,
        },
    )?;

    Ok(())
}
