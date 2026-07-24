//! Module-owned transactional emails via `host::email::send`.

use portaki_sdk::host::email::{
    self, EmailAudience, LocalizedEmailText, ModuleEmailCta, ModuleEmailSdui, SendEmailArgs,
};
use portaki_sdk::prelude::*;
use uuid::Uuid;

use crate::description;
use crate::entities::LostFoundReport;
use crate::storage;

/// Guest self-report → notify workspace owner.
pub fn notify_host_submitted(
    property_id: Uuid,
    stay_id: Uuid,
    kind: &str,
    item_description: &str,
    contact_hint: Option<&str>,
    details: Option<&str>,
) -> Result<()> {
    let mut body = format!(
        "Un voyageur a signalé un objet ({kind}) :\n\n{item_description}"
    );
    if let Some(hint) = contact_hint {
        body.push_str("\n\nContact / lieu : ");
        body.push_str(hint);
    }
    if let Some(extra) = details {
        body.push_str("\n\nDétails : ");
        body.push_str(extra);
    }

    email::send(&SendEmailArgs {
        email_id: format!("submitted-{stay_id}"),
        audience: EmailAudience::Host,
        content: ModuleEmailSdui {
            subject: LocalizedEmailText::new(
                "Un voyageur a signalé un objet perdu/trouvé",
                "A guest reported a lost or found item",
            ),
            eyebrow: Some(LocalizedEmailText::both("Objets perdus / trouvés")),
            title: Some(LocalizedEmailText::new(
                "Nouveau signalement",
                "New report",
            )),
            body: LocalizedEmailText::both(body),
            cta: Some(ModuleEmailCta {
                label: LocalizedEmailText::new("Voir le logement", "View property"),
                url: None,
                portaki_action: None,
            }),
        },
        stay_id: Some(stay_id),
        property_id: Some(property_id),
        action_url: None,
    })
}

/// Host-declared found item → notify guest.
pub fn notify_guest_host_found(stay_id: Uuid, report_id: Uuid, plain_description: &str) -> Result<()> {
    let body = format!(
        "En préparant le logement, l'hôte a retrouvé un objet qui pourrait vous appartenir.\n\n« {plain_description} »\n\nRépondez à cet email ou ouvrez votre livret pour organiser la récupération."
    );
    email::send(&SendEmailArgs {
        email_id: format!("host-found-{report_id}"),
        audience: EmailAudience::Guest,
        content: ModuleEmailSdui {
            subject: LocalizedEmailText::new(
                "Vous avez peut-être oublié quelque chose",
                "You may have left something behind",
            ),
            eyebrow: Some(LocalizedEmailText::both("Objet trouvé")),
            title: Some(LocalizedEmailText::new(
                "Vous avez peut-être oublié quelque chose",
                "You may have left something behind",
            )),
            body: LocalizedEmailText::both(body),
            cta: Some(ModuleEmailCta {
                label: LocalizedEmailText::new("Contacter l'hôte", "Contact the host"),
                url: None,
                portaki_action: Some("open-module:lost-found:default".into()),
            }),
        },
        stay_id: Some(stay_id),
        property_id: None,
        action_url: None,
    })
}

/// J+2 checkout follow-up — only when at least one declaration exists for the stay.
pub fn send_checkout_follow_up(ctx: &Context) -> Result<()> {
    let stay_id = ctx
        .guest
        .as_ref()
        .map(|guest| guest.session_id)
        .ok_or_else(|| PortakiError::Host("stay_id_required".to_string()))?;

    let reports = storage::list_by_stay(stay_id)?;
    if reports.is_empty() {
        return Ok(());
    }

    let Some(joined) = join_descriptions(&reports) else {
        return Ok(());
    };

    let body = format!(
        "En préparant le logement pour les prochains voyageurs, un objet lié à votre séjour a été déclaré.\n\n« {joined} »\n\nRépondez à cet email pour organiser le renvoi ou la récupération."
    );

    email::send(&SendEmailArgs {
        email_id: "checkout-j2".into(),
        audience: EmailAudience::Guest,
        content: ModuleEmailSdui {
            subject: LocalizedEmailText::new(
                "Vous avez peut-être oublié quelque chose",
                "You may have left something behind",
            ),
            eyebrow: Some(LocalizedEmailText::both("Objet trouvé")),
            title: Some(LocalizedEmailText::new(
                "Vous avez peut-être oublié quelque chose",
                "You may have left something behind",
            )),
            body: LocalizedEmailText::both(body),
            cta: Some(ModuleEmailCta {
                label: LocalizedEmailText::new("Contacter l'hôte", "Contact the host"),
                url: None,
                portaki_action: Some("open-module:lost-found:default".into()),
            }),
        },
        stay_id: Some(stay_id),
        property_id: None,
        action_url: None,
    })
}

fn join_descriptions(reports: &[LostFoundReport]) -> Option<String> {
    let parts: Vec<String> = reports
        .iter()
        .map(|row| description::to_plain_text(&row.item_description))
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
        .collect();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" · "))
    }
}
