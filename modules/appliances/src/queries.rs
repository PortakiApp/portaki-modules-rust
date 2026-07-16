//! Module queries — appliance guide content.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::content::{pick_locale, ApplianceDevice, AppliancesPayload};
use crate::store;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetContentArgs {
    pub locale: Option<String>,
    /// Optional device id for detail views (also used when overlay args are forwarded).
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppliancesContentView {
    pub safety_notice: String,
    pub devices: Vec<ApplianceDevice>,
    pub content_fr: String,
    pub content_en: String,
}

#[portaki_sdk::query(name = "getContent")]
pub fn get_content(ctx: Context, args: GetContentArgs) -> Result<AppliancesContentView> {
    let locale = args.locale.unwrap_or_else(|| ctx.locale.clone());
    let row = store::load_content()?;
    let (content_fr, content_en) = match row {
        Some(row) => (row.content_fr, row.content_en),
        None => (String::new(), String::new()),
    };
    let payload = pick_locale(&content_fr, &content_en, &locale);
    Ok(AppliancesContentView {
        safety_notice: payload.safety_notice,
        devices: payload.devices,
        content_fr,
        content_en,
    })
}

pub fn load_payload(ctx: &Context) -> Result<AppliancesPayload> {
    let view = get_content(
        ctx.clone(),
        GetContentArgs {
            locale: Some(ctx.locale.clone()),
            device_id: None,
        },
    )?;
    Ok(AppliancesPayload {
        safety_notice: view.safety_notice,
        devices: view.devices,
    })
}
