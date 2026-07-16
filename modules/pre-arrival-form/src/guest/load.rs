//! Load pre-arrival status for guest surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::surface::Surface;

use super::empty::empty_state_if_module_not_ready;
use crate::storage;

pub enum GuestLoad {
    Empty(Box<Surface>),
    Form,
    Completed,
}

pub fn load_guest_pre_arrival(ctx: &GuestContext) -> Result<GuestLoad> {
    if let Some(surface) = empty_state_if_module_not_ready("home.card")? {
        return Ok(GuestLoad::Empty(Box::new(surface)));
    }

    let Some(guest) = ctx.guest.as_ref() else {
        return Ok(GuestLoad::Form);
    };

    match storage::find_by_stay(guest.session_id)? {
        Some(_) => Ok(GuestLoad::Completed),
        None => Ok(GuestLoad::Form),
    }
}
