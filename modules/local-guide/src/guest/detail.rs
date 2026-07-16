//! Guest explore / bottom-sheet detail surface.

use portaki_sdk::sdui::primitives::Stack;
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use super::body::build_spots_body;
use super::load::GuestData;

pub fn build_detail_surface(data: &GuestData) -> Surface {
    Surface::new(
        Stack::new()
            .gap(json!(12))
            .children(build_spots_body(data, true)),
    )
    .with_id("explore.detail")
}
