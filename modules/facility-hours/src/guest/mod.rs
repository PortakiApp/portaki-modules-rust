//! Guest booklet surfaces.

mod body;
mod detail;
mod empty;
mod home;
mod load;

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::surface::Surface;

use detail::build_detail_surface;
use empty::{empty_runtime_error_state, log_render_failure};
use home::build_home_card;
use load::{load_guest_data, GuestLoad};

#[portaki_sdk::surface(guest, id = "home.card")]
pub fn render_home_card(ctx: GuestContext) -> Surface {
    match render_with_data(&ctx, "home.card", build_home_card) {
        Ok(surface) => surface,
        Err(error) => {
            log_render_failure("home.card", &error);
            empty_runtime_error_state("home.card")
        }
    }
}

#[portaki_sdk::surface(guest, id = "explore.detail")]
pub fn render_explore_detail(ctx: GuestContext) -> Surface {
    match render_with_data(&ctx, "explore.detail", build_detail_surface) {
        Ok(surface) => surface,
        Err(error) => {
            log_render_failure("explore.detail", &error);
            empty_runtime_error_state("explore.detail")
        }
    }
}

fn render_with_data(
    ctx: &GuestContext,
    surface_id: &str,
    build: fn(&load::GuestData) -> Surface,
) -> Result<Surface> {
    match load_guest_data(ctx, surface_id)? {
        GuestLoad::Empty(surface) => Ok(*surface),
        GuestLoad::Ready(data) => Ok(build(&data)),
    }
}
