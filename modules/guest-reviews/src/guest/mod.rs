//! Guest booklet surfaces (inline post-stay — no overlay).

mod empty;
mod home;
mod load;

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::surface::Surface;

use empty::{empty_runtime_error_state, log_render_failure};
use home::build_home_card;
use load::{load_guest_data, GuestLoad};

#[portaki_sdk::surface(guest, id = "home.card")]
pub fn render_home_card(ctx: GuestContext) -> Surface {
    match load_guest_data(&ctx, "home.card") {
        Ok(GuestLoad::Empty(surface)) => *surface,
        Ok(GuestLoad::Ready(data)) => build_home_card(&data),
        Err(error) => {
            log_render_failure("home.card", &error);
            empty_runtime_error_state("home.card")
        }
    }
}
