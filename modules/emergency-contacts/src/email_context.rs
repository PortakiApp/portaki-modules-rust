//! Guest-email host phone for Portaki arrival / post-arrival / lost-found.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::load_config;

/// Arguments for `emailContext`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmailContextArgs {
    #[serde(default)]
    pub template_key: Option<EmailTemplateKey>,
    #[serde(default)]
    pub locale: Option<String>,
}

/// Email-ready emergency-contacts contribution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EmailContextResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_phone: Option<String>,
}

/// Host-visible phone for Portaki guest templates.
#[portaki_sdk::query(name = "emailContext")]
pub fn email_context(ctx: Context, args: EmailContextArgs) -> Result<EmailContextResponse> {
    build_email_context(ctx, args)
}

pub fn build_email_context(_ctx: Context, args: EmailContextArgs) -> Result<EmailContextResponse> {
    match args.template_key {
        None
        | Some(EmailTemplateKey::Arrival)
        | Some(EmailTemplateKey::ArrivalDay)
        | Some(EmailTemplateKey::PostArrival)
        | Some(EmailTemplateKey::LostFound) => {}
        Some(_) => return Ok(EmailContextResponse { host_phone: None }),
    }

    let config = load_config().unwrap_or_default();
    let phone = config.host_visible_phone.trim();
    Ok(EmailContextResponse {
        host_phone: if phone.is_empty() {
            None
        } else {
            Some(phone.to_string())
        },
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
    fn when_arrival_then_returns_host_phone() {
        let (ctx, host) = MockContext::guest()
            .with_capabilities(&[capability::core::STORAGE])
            .with_kv(
                "config",
                serde_json::to_vec(&json!({
                    "host_visible_phone": "+33 6 12 34 56 78"
                }))
                .unwrap(),
            )
            .build();

        with_host(host, ctx.clone(), || {
            let out = build_email_context(
                ctx.clone(),
                EmailContextArgs {
                    template_key: Some(EmailTemplateKey::Arrival),
                    locale: None,
                },
            )
            .unwrap();
            assert_eq!(out.host_phone.as_deref(), Some("+33 6 12 34 56 78"));
        });
    }

    #[test]
    #[serial_test::serial]
    fn when_wrong_template_then_empty() {
        let (ctx, host) = MockContext::guest()
            .with_capabilities(&[capability::core::STORAGE])
            .with_kv(
                "config",
                serde_json::to_vec(&json!({
                    "host_visible_phone": "+33 6 12 34 56 78"
                }))
                .unwrap(),
            )
            .build();

        with_host(host, ctx.clone(), || {
            let out = build_email_context(
                ctx,
                EmailContextArgs {
                    template_key: Some(EmailTemplateKey::StayLink),
                    locale: None,
                },
            )
            .unwrap();
            assert!(out.host_phone.is_none());
        });
    }
}
