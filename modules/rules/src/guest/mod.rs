//! Guest booklet surfaces.

mod empty;
mod home;
mod page;

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::surface::Surface;

use empty::{empty_runtime_error_state, log_render_failure};
use home::build_home_card;
use page::build_detail_page;

use crate::content::RulesPayload;
use crate::queries::load_payload;

/// Guest home booklet card — glance of first rules.
#[portaki_sdk::surface(guest, id = "home.card")]
pub fn render_home_card(ctx: GuestContext) -> Surface {
    match render_with_payload(&ctx, "home.card", build_home_card) {
        Ok(surface) => surface,
        Err(error) => {
            log_render_failure("home.card", &error);
            empty_runtime_error_state("home.card")
        }
    }
}

/// Full rules page (body-only — shell supplies header).
#[portaki_sdk::surface(guest, id = "explore.detail")]
pub fn render_explore_detail(ctx: GuestContext) -> Surface {
    match render_with_payload(&ctx, "explore.detail", build_detail_page) {
        Ok(surface) => surface,
        Err(error) => {
            log_render_failure("explore.detail", &error);
            empty_runtime_error_state("explore.detail")
        }
    }
}

fn render_with_payload(
    ctx: &GuestContext,
    surface_id: &str,
    build: fn(&RulesPayload) -> Surface,
) -> Result<Surface> {
    if let Some(surface) = empty::empty_state_if_module_not_ready(surface_id)? {
        return Ok(surface);
    }
    let payload = load_payload(ctx)?;
    if payload.is_empty() {
        return Ok(empty::empty_content_state(surface_id));
    }
    Ok(build(&payload))
}
