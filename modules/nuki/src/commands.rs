//! Module commands — host config and `access.smart_lock` guest protocol.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{load_config, save_config, ModuleConfig};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StayArgs {
    pub stay_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigArgs {
    #[serde(default)]
    pub smartlock_id: String,
    #[serde(default)]
    pub keypad_code: String,
    #[serde(default)]
    pub device_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestCredentialResponse {
    #[serde(rename = "type")]
    pub credential_type: &'static str,
    pub code: String,
    pub smartlock_id: String,
}

#[derive(Debug, Serialize)]
pub struct UnlockResponse {
    pub ok: bool,
    pub mode: &'static str,
    pub code: String,
}

#[portaki_sdk::command(name = "updateConfig")]
pub fn update_config(_ctx: Context, args: UpdateConfigArgs) -> Result<()> {
    save_config(&ModuleConfig {
        smartlock_id: args.smartlock_id.trim().to_string(),
        keypad_code: args.keypad_code.trim().to_string(),
        device_name: args.device_name.trim().to_string(),
    })
}

#[portaki_sdk::command(name = "getGuestCredential")]
pub fn get_guest_credential(_ctx: Context, _args: StayArgs) -> Result<GuestCredentialResponse> {
    let config = load_config()?;
    let code = require_keypad_code(&config)?;
    Ok(GuestCredentialResponse {
        credential_type: "keypad",
        code,
        smartlock_id: config.smartlock_id.trim().to_string(),
    })
}

#[portaki_sdk::command(name = "unlock")]
pub fn unlock(_ctx: Context, _args: StayArgs) -> Result<UnlockResponse> {
    let config = load_config()?;
    let code = require_keypad_code(&config)?;
    Ok(UnlockResponse {
        ok: true,
        mode: "credential_fallback",
        code,
    })
}

fn require_keypad_code(config: &ModuleConfig) -> Result<String> {
    let code = config.keypad_code_trimmed();
    if code.is_empty() {
        return Err(PortakiError::Host("keypad_code not configured".into()));
    }
    Ok(code.to_string())
}
