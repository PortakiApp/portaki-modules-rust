//! Load config for guest surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::surface::Surface;

use crate::config::{load_config, ModuleConfig, ReviewChannel};

use super::empty::{empty_content_state, empty_state_if_module_not_ready};

pub struct GuestData {
    pub channel: ReviewChannel,
    pub show_qr: bool,
    pub airbnb_url: Option<String>,
    pub thank_you: String,
    pub property_name: String,
    pub locale: String,
}

pub enum GuestLoad {
    Ready(GuestData),
    Empty(Box<Surface>),
}

pub fn load_guest_data(ctx: &GuestContext, surface_id: &str) -> Result<GuestLoad> {
    if let Some(surface) = empty_state_if_module_not_ready(surface_id)? {
        return Ok(GuestLoad::Empty(Box::new(surface)));
    }

    let config = load_config().unwrap_or_else(|_| ModuleConfig::default());
    let airbnb_url = config.airbnb_url();
    let show_airbnb = config.review_channel.show_airbnb() && airbnb_url.is_some();
    let show_portaki = config.review_channel.show_portaki();
    if !show_airbnb && !show_portaki {
        return Ok(GuestLoad::Empty(Box::new(empty_content_state(surface_id))));
    }

    Ok(GuestLoad::Ready(GuestData {
        channel: config.review_channel,
        show_qr: config.show_qr_code && show_airbnb,
        airbnb_url,
        thank_you: config.thank_you_message.trim().to_string(),
        property_name: ctx.property.name.clone(),
        locale: ctx.locale.clone(),
    }))
}
