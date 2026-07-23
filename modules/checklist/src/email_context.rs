//! Guest-email checkout tips for Portaki `lost-found` (and future checkout mails).

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::labels::{get_label, labels_from_item, pick_label};
use crate::storage;

const MAX_TIPS: usize = 3;

/// Arguments for `emailContext`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmailContextArgs {
    #[serde(default)]
    pub template_key: Option<String>,
    #[serde(default)]
    pub locale: Option<String>,
}

/// Email-ready checklist contribution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EmailContextResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_tips: Option<String>,
}

/// Top checklist labels for Portaki guest templates.
#[portaki_sdk::query(name = "emailContext")]
pub fn email_context(ctx: Context, args: EmailContextArgs) -> Result<EmailContextResponse> {
    build_email_context(ctx, args)
}

pub fn build_email_context(ctx: Context, args: EmailContextArgs) -> Result<EmailContextResponse> {
    let template = args.template_key.as_deref().unwrap_or("").trim();
    if !matches!(template, "lost-found" | "post-arrival" | "") {
        return Ok(EmailContextResponse {
            checkout_tips: None,
        });
    }

    let locale = args
        .locale
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(ctx.locale.as_str());
    let property_locale = ctx.property.locale.as_str();

    let mut items = storage::list_items()?;
    items.sort_by_key(|item| item.sort_order);

    let tips: Vec<String> = items
        .iter()
        .filter_map(|item| {
            let labels = labels_from_item(item);
            let label = pick_label(&labels, locale, property_locale);
            let trimmed = label.trim();
            if trimmed.is_empty() {
                let fallback = get_label(item, locale);
                let fallback = fallback.trim();
                if fallback.is_empty() {
                    None
                } else {
                    Some(fallback.to_string())
                }
            } else {
                Some(trimmed.to_string())
            }
        })
        .take(MAX_TIPS)
        .collect();

    if tips.is_empty() {
        return Ok(EmailContextResponse {
            checkout_tips: None,
        });
    }

    Ok(EmailContextResponse {
        checkout_tips: Some(tips.join("\n")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::seed_test_items;
    use chrono::Utc;
    use portaki_test_utils::MockContext;

    #[test]
    fn when_lost_found_then_returns_top_tips() {
        seed_test_items(
            Utc::now(),
            &[
                ("Vider le frigo", "Empty the fridge"),
                ("Fermer les volets", "Close shutters"),
            ],
        );

        let (ctx, _host) = MockContext::guest()
            .with_capabilities(&["core.storage"])
            .build();
        let out = build_email_context(
            ctx,
            EmailContextArgs {
                template_key: Some("lost-found".into()),
                locale: Some("fr".into()),
            },
        )
        .unwrap();
        let tips = out.checkout_tips.expect("tips");
        assert!(tips.contains("Vider le frigo"));
        assert!(tips.contains("Fermer les volets"));
    }
}
