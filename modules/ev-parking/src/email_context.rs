//! Guest-email EV parking hint for Portaki arrival templates.

use portaki_sdk::prelude::*;

use crate::config::load_config;

/// Gateway `emailContext` args — shared SDK wire type.
pub use portaki_sdk::EmailContextArgs;

/// Email-ready ev-parking contribution.
#[portaki_sdk::wire]
#[derive(PartialEq, Eq)]
pub struct EmailContextResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_parking_spot: Option<String>,
}

#[portaki_sdk::query(name = "emailContext")]
pub fn email_context(ctx: Context, args: EmailContextArgs) -> Result<EmailContextResponse> {
    build_email_context(ctx, args)
}

pub fn build_email_context(_ctx: Context, args: EmailContextArgs) -> Result<EmailContextResponse> {
    if !args.allows_template(&[EmailTemplateKey::Arrival, EmailTemplateKey::ArrivalDay]) {
        return Ok(EmailContextResponse {
            ev_parking_spot: None,
        });
    }

    let config = load_config().unwrap_or_default();
    let spot = config.spot_label.trim();
    Ok(EmailContextResponse {
        ev_parking_spot: if spot.is_empty() {
            None
        } else {
            Some(spot.to_string())
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
    fn when_arrival_then_returns_ev_parking_spot() {
        let (ctx, host) = MockContext::guest()
            .with_capabilities(&[capability::core::STORAGE])
            .with_kv(
                "config",
                serde_json::to_vec(&json!({
                    "spot_label": "P2 / Place 14",
                    "charger_pin": "4821"
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
                    ..Default::default()
                },
            )
            .unwrap();
            assert_eq!(out.ev_parking_spot.as_deref(), Some("P2 / Place 14"));
        });
    }

    #[test]
    #[serial_test::serial]
    fn when_wrong_template_then_empty() {
        let (ctx, host) = MockContext::guest()
            .with_capabilities(&[capability::core::STORAGE])
            .with_kv(
                "config",
                serde_json::to_vec(&json!({ "spot_label": "P2 / Place 14" })).unwrap(),
            )
            .build();

        with_host(host, ctx.clone(), || {
            let out = build_email_context(
                ctx,
                EmailContextArgs {
                    template_key: Some(EmailTemplateKey::StayLink),
                    locale: None,
                    ..Default::default()
                },
            )
            .unwrap();
            assert!(out.ev_parking_spot.is_none());
        });
    }
}
