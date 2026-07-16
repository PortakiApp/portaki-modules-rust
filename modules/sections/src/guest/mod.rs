//! Guest booklet surfaces.

mod empty;
mod home;
mod sheet;

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::surface::Surface;

use empty::{empty_runtime_error_state, log_render_failure};
use home::build_home_card;
use sheet::build_sheet_surface;

use crate::model::SectionView;
use crate::queries::{list_sections, ListSectionsArgs};

#[portaki_sdk::surface(guest, id = "home.card")]
pub fn render_home_card(ctx: GuestContext) -> Surface {
    match render_with_sections(&ctx, "home.card", build_home_card) {
        Ok(surface) => surface,
        Err(error) => {
            log_render_failure("home.card", &error);
            empty_runtime_error_state("home.card")
        }
    }
}

#[portaki_sdk::surface(guest, id = "explore.sheet")]
pub fn render_explore_sheet(ctx: GuestContext) -> Surface {
    match render_with_sections(&ctx, "explore.sheet", build_sheet_surface) {
        Ok(surface) => surface,
        Err(error) => {
            log_render_failure("explore.sheet", &error);
            empty_runtime_error_state("explore.sheet")
        }
    }
}

fn render_with_sections(
    ctx: &GuestContext,
    surface_id: &str,
    build: fn(&[SectionView]) -> Surface,
) -> Result<Surface> {
    if let Some(surface) = empty::empty_state_if_module_not_ready(surface_id)? {
        return Ok(surface);
    }
    let sections = list_sections(
        ctx.clone(),
        ListSectionsArgs {
            locale: Some(ctx.locale.clone()),
        },
    )?;
    let visible: Vec<SectionView> = sections.into_iter().filter(|s| !s.is_blank()).collect();
    if visible.is_empty() {
        return Ok(empty::empty_content_state(surface_id));
    }
    Ok(build(&visible))
}
