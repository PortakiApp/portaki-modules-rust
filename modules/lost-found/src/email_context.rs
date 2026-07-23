//! Guest-email tip for Portaki `lost-found` template.

use portaki_sdk::prelude::*;

use crate::config::load_config;

/// Gateway `emailContext` args — shared SDK wire type.
pub use portaki_sdk::EmailContextArgs;

/// Email-ready lost-found contribution.
#[portaki_sdk::wire]
#[derive(PartialEq, Eq)]
pub struct EmailContextResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_tips: Option<String>,
}

#[portaki_sdk::query(name = "emailContext")]
pub fn email_context(ctx: Context, args: EmailContextArgs) -> Result<EmailContextResponse> {
    build_email_context(ctx, args)
}

pub fn build_email_context(_ctx: Context, args: EmailContextArgs) -> Result<EmailContextResponse> {
    if !args.allows_template(&[EmailTemplateKey::LostFound]) {
        return Ok(EmailContextResponse {
            checkout_tips: None,
        });
    }

    let config = load_config().unwrap_or_default();
    Ok(EmailContextResponse {
        checkout_tips: config.host_note_text().map(str::to_string),
    })
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
        });
    }
}
