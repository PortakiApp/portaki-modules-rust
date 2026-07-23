//! Guest-email local tip for Portaki `arrival-day` / `post-arrival`.

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

/// Email-ready local-guide contribution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EmailContextResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_tip: Option<String>,
}

/// One host pick for Portaki guest templates.
#[portaki_sdk::query(name = "emailContext")]
pub fn email_context(ctx: Context, args: EmailContextArgs) -> Result<EmailContextResponse> {
    build_email_context(ctx, args)
}

pub fn build_email_context(ctx: Context, args: EmailContextArgs) -> Result<EmailContextResponse> {
    match args.template_key {
        None | Some(EmailTemplateKey::ArrivalDay) | Some(EmailTemplateKey::PostArrival) => {}
        Some(_) => return Ok(EmailContextResponse { local_tip: None }),
    }

    let locale = args
        .locale
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(ctx.locale.as_str());
    let property_locale = ctx.property.locale.as_str();

    let config = load_config().unwrap_or_default();
    let Some(spot) = config.parse_spots().into_iter().next() else {
        return Ok(EmailContextResponse { local_tip: None });
    };

    let title = spot.title.pick_with_fallback(locale, property_locale);
    let title = title.trim();
    if title.is_empty() {
        return Ok(EmailContextResponse { local_tip: None });
    }

    let mut tip = title.to_string();
    if let Some(distance) = spot
        .distance
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        tip.push_str(" · ");
        tip.push_str(distance);
    } else if let Some(tag) = spot.tag.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        tip.push_str(" · ");
        tip.push_str(tag);
    }

    Ok(EmailContextResponse {
        local_tip: Some(tip),
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
    fn when_arrival_day_then_returns_first_spot() {
        let (ctx, host) = MockContext::guest()
            .with_capabilities(&[capability::core::STORAGE])
            .with_kv(
                "config",
                serde_json::to_vec(&json!({
                    "spots": [{
                        "id": "1",
                        "title": {"fr": "Plage de la Garoupe", "en": "Garoupe beach"},
                        "distance": "8 min à pied",
                        "tag": "Coup de cœur"
                    }]
                }))
                .unwrap(),
            )
            .build();

        with_host(host, ctx.clone(), || {
            let out = build_email_context(
                ctx,
                EmailContextArgs {
                    template_key: Some(EmailTemplateKey::ArrivalDay),
                    locale: Some("fr".into()),
                },
            )
            .unwrap();
            assert_eq!(
                out.local_tip.as_deref(),
                Some("Plage de la Garoupe · 8 min à pied")
            );
        });
    }
}
