//! Module queries — read host configuration (shared + active locale texts).

use portaki_sdk::prelude::*;
use serde::Serialize;

use crate::config::{load_config, ModuleConfig};
use crate::texts::{lang_code, load_texts_for_host, ModuleTexts};

#[derive(Debug, Clone, Serialize)]
pub struct GetConfigResponse {
    pub config: ModuleConfig,
    pub texts: ModuleTexts,
    /// Short language code used for `texts` (`fr`, `en`, …).
    pub lang: String,
}

#[portaki_sdk::query(name = "getConfig")]
pub fn get_config(ctx: Context) -> Result<GetConfigResponse> {
    let lang = lang_code(&ctx.locale);
    let config = load_config()?;
    let texts = load_texts_for_host(&ctx.locale)?;
    Ok(GetConfigResponse {
        config,
        texts,
        lang,
    })
}
