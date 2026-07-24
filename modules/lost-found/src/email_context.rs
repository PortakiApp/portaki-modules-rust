//! Guest-email tip + declaration descriptions for Portaki `lost-found` template.

use portaki_sdk::prelude::*;
use uuid::Uuid;

use crate::config::load_config;
use crate::description;
use crate::entities::LostFoundReport;
use crate::storage;

/// Gateway `emailContext` args — shared SDK wire type.
pub use portaki_sdk::EmailContextArgs;

/// Email-ready lost-found contribution.
#[portaki_sdk::wire]
#[derive(PartialEq, Eq)]
pub struct EmailContextResponse {
    /// Optional host tip (plain extract of `host_note`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_tips: Option<String>,
    /// Joined plain descriptions from stay declarations — empty when none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lost_item_description: Option<String>,
    /// `true` when at least one [`LostFoundReport`] exists for the stay.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_declaration: bool,
}

#[portaki_sdk::query(name = "emailContext")]
pub fn email_context(ctx: Context, args: EmailContextArgs) -> Result<EmailContextResponse> {
    build_email_context(ctx, args)
}

pub fn build_email_context(ctx: Context, args: EmailContextArgs) -> Result<EmailContextResponse> {
    if !args.allows_template(&[EmailTemplateKey::LostFound]) {
        return Ok(EmailContextResponse {
            checkout_tips: None,
            lost_item_description: None,
            has_declaration: false,
        });
    }

    let config = load_config().unwrap_or_default();
    let checkout_tips = config.host_note_text().and_then(|raw| {
        let plain = description::to_plain_text(raw);
        if plain.is_empty() {
            None
        } else {
            Some(plain)
        }
    });

    let stay_id = resolve_stay_id(&ctx, &args);
    let reports = stay_id
        .map(storage::list_by_stay)
        .transpose()?
        .unwrap_or_default();
    let has_declaration = !reports.is_empty();
    let lost_item_description = join_descriptions(&reports);

    Ok(EmailContextResponse {
        checkout_tips,
        lost_item_description,
        has_declaration,
    })
}

fn resolve_stay_id(ctx: &Context, args: &EmailContextArgs) -> Option<Uuid> {
    if let Some(guest) = ctx.guest.as_ref() {
        return Some(guest.session_id);
    }
    if let Some(stay) = ctx.stay.as_ref() {
        return Some(stay.stay_id);
    }
    args.stay_id
        .as_deref()
        .and_then(|raw| Uuid::parse_str(raw.trim()).ok())
}

fn join_descriptions(reports: &[LostFoundReport]) -> Option<String> {
    let parts: Vec<String> = reports
        .iter()
        .filter_map(|report| {
            let plain = description::to_plain_text(&report.item_description);
            let text = if plain.is_empty() {
                report.item_description.trim().to_string()
            } else {
                plain
            };
            if text.is_empty() {
                None
            } else {
                Some(text)
            }
        })
        .collect();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use portaki_sdk::host::with_host;
    use portaki_test_utils::MockContext;
    use serde_json::json;

    #[test]
    #[serial_test::serial]
    fn when_lost_found_then_returns_host_note() {
        let (ctx, host) = MockContext::guest()
            .with_capabilities(&[capability::core::STORAGE])
            .with_kv(
                "config",
                serde_json::to_vec(&json!({ "host_note": "Check the lobby closet." })).unwrap(),
            )
            .build();

        with_host(host, ctx.clone(), || {
            let out = build_email_context(
                ctx,
                EmailContextArgs {
                    template_key: Some(EmailTemplateKey::LostFound),
                    locale: None,
                    ..Default::default()
                },
            )
            .unwrap();
            assert_eq!(
                out.checkout_tips.as_deref(),
                Some("Check the lobby closet.")
            );
            assert!(!out.has_declaration);
            assert!(out.lost_item_description.is_none());
        });
    }

    #[test]
    #[serial_test::serial]
    fn when_wrong_template_then_empty() {
        let (ctx, host) = MockContext::guest()
            .with_capabilities(&[capability::core::STORAGE])
            .with_kv(
                "config",
                serde_json::to_vec(&json!({ "host_note": "Tip" })).unwrap(),
            )
            .build();

        with_host(host, ctx.clone(), || {
            let out = build_email_context(
                ctx,
                EmailContextArgs {
                    template_key: Some(EmailTemplateKey::Arrival),
                    locale: None,
                    ..Default::default()
                },
            )
            .unwrap();
            assert!(out.checkout_tips.is_none());
            assert!(out.lost_item_description.is_none());
            assert!(!out.has_declaration);
        });
    }
}
