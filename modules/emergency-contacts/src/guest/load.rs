//! Load config for guest surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::surface::Surface;

use crate::config::{load_config, ContactRow, ModuleConfig};

use super::empty::{empty_content_state, empty_state_if_module_not_ready};

pub struct GuestData {
    pub contacts: Vec<ContactRow>,
    pub host_phone: String,
    pub locale: String,
}

pub enum GuestLoad {
    Ready(GuestData),
    Empty(Surface),
}

pub fn load_guest_data(ctx: &GuestContext, surface_id: &str) -> Result<GuestLoad> {
    if let Some(surface) = empty_state_if_module_not_ready(surface_id)? {
        return Ok(GuestLoad::Empty(surface));
    }

    let config = load_config().unwrap_or_else(|_| ModuleConfig::default());
    if config.is_empty() {
        return Ok(GuestLoad::Empty(empty_content_state(surface_id)));
    }

    Ok(GuestLoad::Ready(GuestData {
        contacts: config.parse_contacts(),
        host_phone: config.host_visible_phone.trim().to_string(),
        locale: ctx.locale.clone(),
    }))
}
